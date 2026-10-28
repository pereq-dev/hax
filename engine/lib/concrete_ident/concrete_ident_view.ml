open! Prelude
include Concrete_ident_view_types

(** Rust paths come with invariants (e.g. a function is always a `ValueNs _`),
    this function raises an error if a path doesn't respect those. *)
let broken_invariant (type t) msg (did : Explicit_def_id.t) : t =
  let msg =
    "Explicit_def_id: an invariant has been broken. Expected " ^ msg
    ^ ".\n\ndid="
    ^ [%show: Explicit_def_id.t] did
  in
  Stdio.prerr_endline msg;
  failwith msg

(** Helper module to asserts various properties about a DefId. *)
module Assert = struct
  let parent did =
    Explicit_def_id.parent did
    |> Option.value_or_thunk ~default:(fun _ ->
        broken_invariant "the Explicit_def_id to have a parent" did)

  let type_ns (did : Explicit_def_id.t) =
    match List.last (Explicit_def_id.to_def_id did).path with
    | Some { data = TypeNs data; disambiguator } ->
        DisambiguatedString.{ data; disambiguator }
    | _ -> broken_invariant "last path chunk to exist and be of type TypeNs" did

  let macro_ns (did : Explicit_def_id.t) =
    match List.last (Explicit_def_id.to_def_id did).path with
    | Some { data = MacroNs data; disambiguator } ->
        DisambiguatedString.{ data; disambiguator }
    | _ ->
        broken_invariant "last path chunk to exist and be of type MacroNs" did

  let value_ns (did : Explicit_def_id.t) =
    match List.last (Explicit_def_id.to_def_id did).path with
    | Some { data = ValueNs data; disambiguator } ->
        DisambiguatedString.{ data; disambiguator }
    | _ ->
        broken_invariant "last path chunk to exist and be of type ValueNs" did
end

let rec poly :
    'n 'd.
    into_n:(Explicit_def_id.t -> DisambiguatedString.t -> 'n) ->
    into_d:(Explicit_def_id.t -> Int64.t -> 'd) ->
    Explicit_def_id.t ->
    ('n, 'd) RelPath.Chunk.poly =
 fun ~into_n ~into_d did ->
  let poly = poly ~into_n ~into_d in
  let mk_associated_item kind : ('n, 'd) RelPath.Chunk.poly =
    `AssociatedItem
      ( kind,
        match Assert.parent did |> poly with
        | (`Impl _ | `Trait _) as p -> p
        | _ -> broken_invariant "Impl or Trait" (Assert.parent did) )
  in
  let assert_type_ns did = Assert.type_ns did |> into_n did in
  let assert_value_ns did = Assert.value_ns did |> into_n did in
  let assert_macro_ns did = Assert.macro_ns did |> into_n did in
  let result =
    match (Explicit_def_id.to_def_id did).kind with
    | (Ctor (Struct, _) | Struct) when Explicit_def_id.is_constructor did ->
        let name = assert_type_ns did in
        `Constructor (name, `Struct name)
    | Variant | Ctor _ ->
        let parent = Assert.parent did in
        let name = assert_type_ns did in
        `Constructor
          ( name,
            match poly parent with
            | (`Enum _ | `Struct _ | `Union _) as p -> p
            | _ -> broken_invariant "Enum, Struct or Union" parent )
    | Fn -> `Fn (assert_value_ns did)
    | Const -> `Const (assert_value_ns did)
    | AssocFn -> `Fn (assert_value_ns did) |> mk_associated_item
    | AssocConst -> `Const (assert_value_ns did) |> mk_associated_item
    | AssocTy -> `Type (assert_type_ns did) |> mk_associated_item
    | TyAlias -> `TyAlias (assert_type_ns did)
    | Field ->
        let constructor =
          let parent = Assert.parent did in
          match parent |> poly with
          | `Constructor _ as p -> p
          | _ -> broken_invariant "Constructor" parent
        in
        `Field (assert_value_ns did, constructor)
    | Trait -> `Trait (assert_type_ns did, None)
    | TraitAlias -> `Trait (assert_type_ns did, Some `Alias)
    | Macro _ -> `Macro (assert_macro_ns did)
    | Union -> `Union (assert_type_ns did)
    | Enum -> `Enum (assert_type_ns did)
    | Struct -> `Struct (assert_type_ns did)
    | AnonConst ->
        `AnonConst
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = AnonConst; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be AnonConst" did)
    | Closure ->
        `AnonConst
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = Closure; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be Closure" did)
    | Impl { of_trait } ->
        `Impl
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = Impl; disambiguator } ->
              ( into_d did disambiguator,
                (if of_trait then `Trait else `Inherent),
                Explicit_def_id.ImplInfoStore.lookup_raw did )
          | _ -> broken_invariant "last path chunk to be Impl" did)
    | OpaqueTy ->
        `Opaque
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = OpaqueTy; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be Opaque" did)
    | Use ->
        `Use
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = Use; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be Use" did)
    | ForeignMod ->
        `Foreign
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = ForeignMod; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be ForeignMod" did)
    | ForeignTy -> `ForeignTy (assert_type_ns did)
    | ExternCrate -> `ExternCrate (assert_type_ns did)
    | Static _ -> `Static (assert_value_ns did)
    | Mod -> `Mod (assert_type_ns did)
    | GlobalAsm ->
        `GlobalAsm
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = GlobalAsm; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be GlobalAsm" did)
    | InlineConst ->
        `InlineConst
          (match List.last_exn (Explicit_def_id.to_def_id did).path with
          | { data = AnonConst; disambiguator } -> into_d did disambiguator
          | _ -> broken_invariant "last path chunk to be AnonConst" did)
    | TyParam | ConstParam | PromotedConst | LifetimeParam
    | SyntheticCoroutineBody ->
        (* It should be impossible for such items to ever be referenced by anyting in hax. *)
        broken_invariant
          "non (TyParam | ConstParam | InlineConst | PromotedConst | \
           LifetimeParam | SyntheticCoroutineBody) identifier"
          did
  in
  result

let view_name : Explicit_def_id.t -> RelPath.Chunk.t =
  poly ~into_n:(fun _ n -> n) ~into_d:(fun _ d -> d)

let view_name_did : Explicit_def_id.t -> _ RelPath.Chunk.poly =
  let mk x y = (x, y) in
  poly ~into_n:mk ~into_d:mk

let of_def_id (did : Explicit_def_id.t) : t =
  (* We distinguish between:
     - a chain of identifiers that have a relation with each other (e.g. if `k::E::C` is a constructor and `k::E` a enum)
     - a chain of identifiers that have no relation (e.g. `k::f` and `k::f::g` are both functions).
  *)
  (* This distinguishing is implemented by `poly` (or `view_name_did` and `view_name`) *)
  (* From `poly`, we can inspect the root of the chain of identifiers, e.g. `k::E` is the root of `k::E::C`. *)
  let ns_chunks, rel_path =
    let rec find name_chunks (did : Explicit_def_id.t) =
      let is_mod did =
        [%matches? (Types.Mod : Types.def_kind)]
          (Explicit_def_id.to_def_id did).kind
      in
      (let*? _did_is_a_mod = is_mod did in
       let parents = Explicit_def_id.parents did in
       let*? _parents_all_mods = List.for_all ~f:is_mod parents in
       Some (List.rev parents, name_chunks))
      |> Option.value_or_thunk ~default:(fun _ ->
          let view = view_name_did did in
          let did =
            view |> RelPath.Chunk.map_poly fst fst |> RelPath.Chunk.root
          in
          let name_chunks =
            RelPath.Chunk.map_poly snd snd view :: name_chunks
          in
          match Explicit_def_id.parent did with
          | None -> ([], name_chunks)
          | Some did -> find name_chunks did)
    in
    find [] did
  in
  let mod_path : DisambiguatedString.t list =
    { data = (Explicit_def_id.to_def_id did).krate; disambiguator = Int64.zero }
    :: List.map
         ~f:(fun (m : Explicit_def_id.t) ->
           match (Explicit_def_id.to_def_id m).path |> List.last_exn with
           | Types.{ disambiguator; data = TypeNs data } ->
               DisambiguatedString.{ data; disambiguator }
           | _ ->
               broken_invariant
                 "A `Mod` identifier must a `TypeNs` as its last path" m)
         ns_chunks
  in
  (* This is a hack: we remove a prefix that we add in
     https://github.com/pereq-dev/hax/blob/02d67770f2626e4bb27fc2a1ba9cfe612819d4a8/hax-lib/macros/src/implementation.rs#L897 *)
  let mod_path =
    List.filter mod_path ~f:(fun ds ->
        String.is_prefix ds.data ~prefix:"hax__autogenerated_refinement_" |> not)
  in
  { rel_path; mod_path }
