module New_tests.Rustc_coverage__unicode
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let v_申し訳ございません (_: Prims.unit) : bool = Core_models.Hint.black_box #bool false

let v_サビ (_: Prims.unit) : Prims.unit = ()

/// @fail(extraction): ssprove(HAX0001)
let main (_: Prims.unit) : Prims.unit =
  let _:Prims.unit =
    Core_models.Iter.Traits.Iterator.f_fold (Core_models.Iter.Traits.Collect.f_into_iter #(Core_models.Ops.Range.t_RangeInclusive
            FStar.Char.char)
          #FStar.Tactics.Typeclasses.solve
          (Core_models.Ops.Range.impl_7__new #FStar.Char.char 'Ð' 'Ð'
            <:
            Core_models.Ops.Range.t_RangeInclusive FStar.Char.char)
        <:
        Core_models.Ops.Range.t_RangeInclusive FStar.Char.char)
      ()
      (fun temp_0_ e_İ ->
          let _:Prims.unit = temp_0_ in
          let e_İ:FStar.Char.char = e_İ in
          ())
  in
  let _:Prims.unit =
    if v_申し訳ございません () && v_申し訳ございません ()
    then
      let _:Prims.unit =
        Std.Io.Stdio.e_print (Core_models.Fmt.Rt.impl_1__new_const (mk_usize 1)
              (let list = ["true\n"] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
            <:
            Core_models.Fmt.t_Arguments)
      in
      let _:Prims.unit = () in
      ()
  in
  let _:Prims.unit = v_サビ () in
  ()

let v_他 (_: Prims.unit) : Prims.unit = ()
