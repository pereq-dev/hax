use crate::prelude::*;

pub const HAX_DRIVER_STDERR_PREFIX: &str = "::hax-driver::";

#[derive_group(Serializers)]
#[derive(Debug, Clone)]
pub struct EmitHaxMetaMessage {
    pub working_dir: Option<PathBuf>,
    pub manifest_dir: Option<PathBuf>,
    pub path: PathBuf,
}
#[derive_group(Serializers)]
#[derive(Debug, Clone)]
pub enum HaxDriverMessage {
    EmitHaxMeta(EmitHaxMetaMessage),
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Clone)]
pub enum Items<Body: hax_frontend_exporter::IsBody> {
    FullDef(Vec<hax_frontend_exporter::FullDef<Body>>),
    Legacy(Vec<hax_frontend_exporter::Item<Body>>),
}

#[derive_group(Serializers)]
#[derive(Debug, Clone)]
pub struct HaxMeta<Body: hax_frontend_exporter::IsBody> {
    pub crate_name: String,
    pub cg_metadata: String,
    pub externs: Vec<PathBuf>,
    pub items: Items<Body>,
    pub impl_infos: Vec<(
        hax_frontend_exporter::DefId,
        hax_frontend_exporter::ImplInfos,
    )>,
    pub def_ids: Vec<hax_frontend_exporter::DefId>,
    pub comments: Vec<(hax_frontend_exporter::Span, String)>,
    pub hax_version: String,
}

use hax_frontend_exporter::id_table;

impl<Body: hax_frontend_exporter::IsBody> HaxMeta<Body>
where
    Body: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    #[tracing::instrument(level = "trace", skip(self, write, id_table))]
    pub fn write(self, write: &mut impl std::io::Write, id_table: id_table::Table) {
        let mut write = zstd::stream::write::Encoder::new(write, 0).unwrap();

        id_table::WithTable::run(id_table, self, |with_table| {
            serde_brief::to_writer(with_table, &mut write).unwrap();
            write.finish().unwrap();
        })
    }
    #[tracing::instrument(level = "trace", skip(reader))]
    pub fn read(reader: impl std::io::Read) -> (Self, id_table::Table) {
        let reader = zstd::stream::read::Decoder::new(reader).unwrap();
        let reader = std::io::BufReader::new(reader);
        let haxmeta = id_table::WithTable::<HaxMeta<Body>>::destruct(
            serde_brief::from_reader(reader).unwrap(),
        );
        if haxmeta.0.hax_version != crate::HAX_VERSION {
            let version = haxmeta.0.hax_version;
            let expected = crate::HAX_VERSION;
            panic!(
                "An invariant was broken: `*.haxmeta` was produced by hax version `{version}` while the current version of hax is `{expected}`. Please report this to https://github.com/pereq-dev/hax/issues."
            );
        };
        haxmeta
    }
}

#[macro_export]
macro_rules! with_kind_type {
    ($kind:expr, <$t:ident>|| $body:expr) => {{
        mod from {
            pub use hax_types::cli_options::ExportBodyKind::{MirBuilt as MB, Thir as T};
        }
        mod to {
            pub type T = hax_frontend_exporter::ThirBody;
            pub type MB = hax_frontend_exporter::MirBody<hax_frontend_exporter::mir_kinds::Built>;
        }
        let mut kind: Vec<::hax_types::cli_options::ExportBodyKind> = $kind;
        kind.sort();
        kind.dedup();
        match kind.as_slice() {
            [from::MB] => {
                type $t = to::MB;
                $body
            }
            [from::T] => {
                type $t = to::T;
                $body
            }
            [from::T, from::MB] => {
                type $t = (to::MB, to::T);
                $body
            }
            [] => {
                type $t = ();
                $body
            }
            _ => panic!("Unsupported kind {:#?}", kind),
        }
    }};
}
pub use with_kind_type;
