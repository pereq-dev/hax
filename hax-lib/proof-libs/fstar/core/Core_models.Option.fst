module Core_models.Option
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Rust_primitives

include Core_models.Bundle {t_Option as t_Option}

include Core_models.Bundle {Option_Some as Option_Some}

include Core_models.Bundle {Option_None as Option_None}

include Core_models.Bundle {impl__is_some as impl__is_some}

include Core_models.Bundle {impl__is_some_and as impl__is_some_and}

include Core_models.Bundle {impl__is_none as impl__is_none}

include Core_models.Bundle {impl__is_none_or as impl__is_none_or}

include Core_models.Bundle {impl__as_ref as impl__as_ref}

include Core_models.Bundle {impl__expect as impl__expect}

include Core_models.Bundle {impl__unwrap as impl__unwrap}

include Core_models.Bundle {impl__unwrap_or as impl__unwrap_or}

include Core_models.Bundle {impl__unwrap_or_else as impl__unwrap_or_else}

include Core_models.Bundle {impl__unwrap_or_default as impl__unwrap_or_default}

include Core_models.Bundle {impl__map as impl__map}

include Core_models.Bundle {impl__map_or as impl__map_or}

include Core_models.Bundle {impl__map_or_else as impl__map_or_else}

include Core_models.Bundle {impl__map_or_default as impl__map_or_default}

include Core_models.Bundle {impl__ok_or as impl__ok_or}

include Core_models.Bundle {impl__ok_or_else as impl__ok_or_else}

include Core_models.Bundle {impl__and_then as impl__and_then}

include Core_models.Bundle {impl__take as impl__take}

include Core_models.Bundle {impl__filter as impl__filter}

include Core_models.Bundle {impl__or as impl__or}

include Core_models.Bundle {impl__or_else as impl__or_else}

include Core_models.Bundle {impl__xor as impl__xor}

include Core_models.Bundle {impl__zip as impl__zip}

include Core_models.Bundle {impl__inspect as impl__inspect}

include Core_models.Bundle {impl_1__flatten as impl_1__flatten}

include Core_models.Bundle {impl_2__cloned as impl_2__cloned}

include Core_models.Bundle {impl_3__from__option as impl_3}

include Core_models.Bundle {impl_4__from__option as impl_4}

include Core_models.Bundle {impl_5 as impl_5}

include Core_models.Bundle {impl_6__from__option as impl_6}
