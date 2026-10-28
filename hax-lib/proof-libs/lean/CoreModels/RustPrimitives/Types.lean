module
public import Aeneas

@[expose] public section

/-- [rust_primitives::sequence::Seq]
    Source: 'rust_primitives/src/lib.rs', lines 47:4-47:21
    Name pattern: [rust_primitives::sequence::Seq]
    Visibility: public -/
@[rust_type "rust_primitives::sequence::Seq"]
def rust_primitives.sequence.Seq (T : Type _) := Aeneas.Std.Slice T

def alloc.string.String := _root_.String
