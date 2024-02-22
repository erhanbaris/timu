
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Welcome10 {
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "one-byte")]
    pub one_byte: Vec<OneByte>,

    #[serde(rename = "two-byte")]
    pub two_byte: TwoByte,

    #[serde(rename = "gen_notes")]
    pub gen_notes: Vec<Note>,

    #[serde(rename = "ring_notes")]
    pub ring_notes: Vec<Note>,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct OneByte {
    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "entry")]
    pub entry: OneByteEntry,

    #[serde(rename = "proc_start")]
    pub proc_start: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleEntry {
    #[serde(rename = "syntax")]
    pub syntax: HilariousSyntax,

    #[serde(rename = "grp1")]
    pub grp1: Option<PurpleGrp1>,

    #[serde(rename = "grp2")]
    pub grp2: Option<TentacledGrp2>,

    #[serde(rename = "note")]
    pub note: Option<PurpleNote>,

    #[serde(rename = "attr")]
    pub attr: Option<PurpleAttr>,

    #[serde(rename = "mode")]
    pub mode: Option<Mode>,

    #[serde(rename = "proc_start")]
    pub proc_start: Option<TentacledProcStart>,

    #[serde(rename = "doc1632_ref")]
    pub doc1632_ref: Option<String>,

    #[serde(rename = "proc_end")]
    pub proc_end: Option<String>,

    #[serde(rename = "ref")]
    pub entry_ref: Option<String>,

    #[serde(rename = "grp3")]
    pub grp3: Option<PurpleGrp3>,

    #[serde(rename = "test_f")]
    pub test_f: Option<TestF>,

    #[serde(rename = "modif_f")]
    pub modif_f: Option<PurpleFF>,

    #[serde(rename = "def_f")]
    pub def_f: Option<PurpleFF>,

    #[serde(rename = "undef_f")]
    pub undef_f: Option<UndefF>,

    #[serde(rename = "direction")]
    pub direction: Option<String>,

    #[serde(rename = "r")]
    pub r: Option<Lock>,

    #[serde(rename = "ring")]
    pub ring: Option<String>,

    #[serde(rename = "f_vals")]
    pub f_vals: Option<FVals>,

    #[serde(rename = "is_undoc")]
    pub is_undoc: Option<Lock>,

    #[serde(rename = "doc")]
    pub doc: Option<Doc>,

    #[serde(rename = "is_doc")]
    pub is_doc: Option<Lock>,

    #[serde(rename = "particular")]
    pub particular: Option<Lock>,

    #[serde(rename = "instr_ext")]
    pub instr_ext: Option<InstrExt>,

    #[serde(rename = "op_size")]
    pub op_size: Option<String>,

    #[serde(rename = "ring_ref")]
    pub ring_ref: Option<RingRef>,

    #[serde(rename = "lock")]
    pub lock: Option<Lock>,

    #[serde(rename = "opcd_ext")]
    pub opcd_ext: Option<String>,

    #[serde(rename = "alias")]
    pub alias: Option<String>,

    #[serde(rename = "sign-ext")]
    pub sign_ext: Option<String>,

    #[serde(rename = "doc_ref")]
    pub doc_ref: Option<String>,

    #[serde(rename = "pref")]
    pub pref: Option<Pref>,

    #[serde(rename = "modif_f_fpu")]
    pub modif_f_fpu: Option<String>,

    #[serde(rename = "undef_f_fpu")]
    pub undef_f_fpu: Option<String>,

    #[serde(rename = "sec_opcd")]
    pub sec_opcd: Option<String>,

    #[serde(rename = "mem_format")]
    pub mem_format: Option<String>,

    #[serde(rename = "def_f_fpu")]
    pub def_f_fpu: Option<String>,

    #[serde(rename = "fpop")]
    pub fpop: Option<Fpop>,

    #[serde(rename = "fpush")]
    pub fpush: Option<Lock>,

    #[serde(rename = "mod")]
    pub entry_mod: Option<Mod>,

    #[serde(rename = "part_alias")]
    pub part_alias: Option<String>,

    #[serde(rename = "doc_part_alias_ref")]
    pub doc_part_alias_ref: Option<String>,

    #[serde(rename = "f_vals_fpu")]
    pub f_vals_fpu: Option<String>,

    #[serde(rename = "doc64_ref")]
    pub doc64_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DefFClass {
    #[serde(rename = "cond")]
    pub cond: Lock,

    #[serde(rename = "#text")]
    pub text: DefF,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleNote {
    #[serde(rename = "brief")]
    pub brief: PurpleBrief,
}

#[derive(Serialize, Deserialize)]
pub struct BriefClass {
    #[serde(rename = "#text")]
    pub text: Vec<String>,

    #[serde(rename = "sub")]
    pub sub: Option<String>,

    #[serde(rename = "sup")]
    pub sup: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleProcStart {
    #[serde(rename = "post")]
    pub post: Option<Post>,

    #[serde(rename = "#text")]
    pub text: String,

    #[serde(rename = "lat_step")]
    pub lat_step: Option<Lock>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSyntax {
    #[serde(rename = "mnem")]
    pub mnem: String,

    #[serde(rename = "dst")]
    pub dst: Option<SrcDst>,

    #[serde(rename = "src")]
    pub src: Option<DstUnion>,

    #[serde(rename = "mod")]
    pub syntax_mod: Option<Mod>,
}

#[derive(Serialize, Deserialize)]
pub struct SrcDst {
    #[serde(rename = "type")]
    pub dst_type: Option<TypeEnum>,

    #[serde(rename = "address")]
    pub address: Option<Address>,

    #[serde(rename = "depend")]
    pub depend: Option<Post>,

    #[serde(rename = "#text")]
    pub text: Option<String>,

    #[serde(rename = "displayed")]
    pub displayed: Option<Post>,

    #[serde(rename = "nr")]
    pub nr: Option<String>,

    #[serde(rename = "group")]
    pub group: Option<Group>,

    #[serde(rename = "a")]
    pub a: Option<A>,

    #[serde(rename = "t")]
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySyntax {
    #[serde(rename = "mnem")]
    pub mnem: MnemUnion,

    #[serde(rename = "dst")]
    pub dst: Option<DstUnion>,

    #[serde(rename = "src")]
    pub src: Option<DstUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct MnemClass {
    #[serde(rename = "sug")]
    pub sug: Lock,

    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyEntry {
    #[serde(rename = "direction")]
    pub direction: Option<String>,

    #[serde(rename = "op_size")]
    pub op_size: Option<String>,

    #[serde(rename = "r")]
    pub r: Option<Lock>,

    #[serde(rename = "lock")]
    pub lock: Option<Lock>,

    #[serde(rename = "syntax")]
    pub syntax: AmbitiousSyntax,

    #[serde(rename = "grp1")]
    pub grp1: PurpleGrp1,

    #[serde(rename = "grp2")]
    pub grp2: Option<StickyGrp2>,

    #[serde(rename = "grp3")]
    pub grp3: Option<PurpleGrp3>,

    #[serde(rename = "modif_f")]
    pub modif_f: Option<FluffyFF>,

    #[serde(rename = "def_f")]
    pub def_f: Option<FluffyFF>,

    #[serde(rename = "note")]
    pub note: FluffyNote,

    #[serde(rename = "attr")]
    pub attr: Option<FluffyAttr>,

    #[serde(rename = "undef_f")]
    pub undef_f: Option<UndefF>,

    #[serde(rename = "f_vals")]
    pub f_vals: Option<String>,

    #[serde(rename = "test_f")]
    pub test_f: Option<String>,

    #[serde(rename = "mode")]
    pub mode: Option<Mode>,

    #[serde(rename = "proc_start")]
    pub proc_start: Option<String>,

    #[serde(rename = "sign-ext")]
    pub sign_ext: Option<String>,

    #[serde(rename = "ring")]
    pub ring: Option<String>,

    #[serde(rename = "ring_ref")]
    pub ring_ref: Option<RingRef>,

    #[serde(rename = "tttn")]
    pub tttn: Option<String>,

    #[serde(rename = "doc64_ref")]
    pub doc64_ref: Option<String>,

    #[serde(rename = "opcd_ext")]
    pub opcd_ext: Option<String>,

    #[serde(rename = "alias")]
    pub alias: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyNote {
    #[serde(rename = "brief")]
    pub brief: String,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledSyntax {
    #[serde(rename = "mnem")]
    pub mnem: String,

    #[serde(rename = "dst")]
    pub dst: Option<TentacledDst>,

    #[serde(rename = "src")]
    pub src: Option<DstUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleDst {
    #[serde(rename = "a")]
    pub a: A,

    #[serde(rename = "t")]
    pub t: TypeEnum,
}

#[derive(Serialize, Deserialize)]
pub struct TwoByte {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "pri_opcd")]
    pub pri_opcd: Vec<PriOpcd>,
}

#[derive(Serialize, Deserialize)]
pub struct PriOpcd {
    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "proc_start")]
    pub proc_start: Option<String>,

    #[serde(rename = "entry")]
    pub entry: PriOpcdEntry,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledEntry {
    #[serde(rename = "mode")]
    pub mode: Option<Mode>,

    #[serde(rename = "opcd_ext")]
    pub opcd_ext: Option<String>,

    #[serde(rename = "syntax")]
    pub syntax: CunningSyntax,

    #[serde(rename = "grp1")]
    pub grp1: Option<FluffyGrp1>,

    #[serde(rename = "note")]
    pub note: TentacledNote,

    #[serde(rename = "attr")]
    pub attr: Option<TentacledAttr>,

    #[serde(rename = "ring")]
    pub ring: Option<String>,

    #[serde(rename = "modif_f")]
    pub modif_f: Option<DefF>,

    #[serde(rename = "def_f")]
    pub def_f: Option<DefF>,

    #[serde(rename = "proc_start")]
    pub proc_start: Option<TentacledProcStart>,

    #[serde(rename = "grp2")]
    pub grp2: Option<IndigoGrp2>,

    #[serde(rename = "doc_ref")]
    pub doc_ref: Option<DocRef>,

    #[serde(rename = "sec_opcd")]
    pub sec_opcd: Option<SecOpcdUnion>,

    #[serde(rename = "instr_ext")]
    pub instr_ext: Option<InstrExt>,

    #[serde(rename = "ring_ref")]
    pub ring_ref: Option<String>,

    #[serde(rename = "doc")]
    pub doc: Option<Doc>,

    #[serde(rename = "doc1632_ref")]
    pub doc1632_ref: Option<String>,

    #[serde(rename = "particular")]
    pub particular: Option<Lock>,

    #[serde(rename = "proc_end")]
    pub proc_end: Option<String>,

    #[serde(rename = "doc64_ref")]
    pub doc64_ref: Option<String>,

    #[serde(rename = "grp3")]
    pub grp3: Option<FluffyGrp3>,

    #[serde(rename = "r")]
    pub r: Option<Lock>,

    #[serde(rename = "pref")]
    pub pref: Option<String>,

    #[serde(rename = "mod")]
    pub entry_mod: Option<Mod>,

    #[serde(rename = "is_undoc")]
    pub is_undoc: Option<Lock>,

    #[serde(rename = "undef_f")]
    pub undef_f: Option<DefF>,

    #[serde(rename = "is_doc")]
    pub is_doc: Option<Lock>,

    #[serde(rename = "f_vals")]
    pub f_vals: Option<String>,

    #[serde(rename = "lock")]
    pub lock: Option<Lock>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledNote {
    #[serde(rename = "brief")]
    pub brief: FluffyBrief,

    #[serde(rename = "det")]
    pub det: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SecOpcdClass {
    #[serde(rename = "escape")]
    pub escape: Lock,

    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct StickySyntax {
    #[serde(rename = "mod")]
    pub syntax_mod: Option<Mod>,

    #[serde(rename = "mnem")]
    pub mnem: String,

    #[serde(rename = "dst")]
    pub dst: StickyDst,

    #[serde(rename = "src")]
    pub src: PurpleSrc,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyDst {
    #[serde(rename = "depend")]
    pub depend: Option<Post>,

    #[serde(rename = "a")]
    pub a: A,

    #[serde(rename = "t")]
    pub t: TypeEnum,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoSyntax {
    #[serde(rename = "mnem")]
    pub mnem: String,

    #[serde(rename = "dst")]
    pub dst: Option<DstUnion>,

    #[serde(rename = "src")]
    pub src: Option<FluffySrc>,
}

#[derive(Serialize, Deserialize)]
pub struct SrcClass {
    #[serde(rename = "a")]
    pub a: Option<A>,

    #[serde(rename = "t")]
    pub t: Option<PurpleT>,

    #[serde(rename = "group")]
    pub group: Option<Group>,

    #[serde(rename = "displayed")]
    pub displayed: Option<Post>,

    #[serde(rename = "#text")]
    pub text: Option<String>,

    #[serde(rename = "depend")]
    pub depend: Option<Post>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyEntry {
    #[serde(rename = "r")]
    pub r: Option<Lock>,

    #[serde(rename = "mode")]
    pub mode: Option<String>,

    #[serde(rename = "proc_start")]
    pub proc_start: StickyProcStart,

    #[serde(rename = "syntax")]
    pub syntax: MagentaSyntax,

    #[serde(rename = "grp1")]
    pub grp1: Option<PurpleGrp1>,

    #[serde(rename = "modif_f")]
    pub modif_f: Option<DefF>,

    #[serde(rename = "def_f")]
    pub def_f: Option<DefF>,

    #[serde(rename = "note")]
    pub note: FluffyNote,

    #[serde(rename = "ring")]
    pub ring: Option<String>,

    #[serde(rename = "attr")]
    pub attr: Option<String>,

    #[serde(rename = "grp2")]
    pub grp2: Option<TentacledGrp2>,

    #[serde(rename = "doc")]
    pub doc: Option<Doc>,

    #[serde(rename = "doc_ref")]
    pub doc_ref: Option<String>,

    #[serde(rename = "ring_ref")]
    pub ring_ref: Option<String>,

    #[serde(rename = "doc64_ref")]
    pub doc64_ref: Option<Doc64Ref>,

    #[serde(rename = "grp3")]
    pub grp3: Option<FluffyGrp3>,

    #[serde(rename = "instr_ext")]
    pub instr_ext: Option<InstrExt>,

    #[serde(rename = "tttn")]
    pub tttn: Option<String>,

    #[serde(rename = "test_f")]
    pub test_f: Option<String>,

    #[serde(rename = "pref")]
    pub pref: Option<String>,

    #[serde(rename = "opcd_ext")]
    pub opcd_ext: Option<String>,

    #[serde(rename = "undef_f")]
    pub undef_f: Option<String>,

    #[serde(rename = "direction")]
    pub direction: Option<String>,

    #[serde(rename = "particular")]
    pub particular: Option<Lock>,

    #[serde(rename = "lock")]
    pub lock: Option<Lock>,

    #[serde(rename = "op_size")]
    pub op_size: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyProcStart {
    #[serde(rename = "lat_step")]
    pub lat_step: Lock,

    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentSyntax {
    #[serde(rename = "mod")]
    pub syntax_mod: Option<Mod>,

    #[serde(rename = "mnem")]
    pub mnem: String,

    #[serde(rename = "dst")]
    pub dst: Option<FluffyDst>,

    #[serde(rename = "src")]
    pub src: Option<PurpleDst>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneByteEntry {
    FluffyEntry(FluffyEntry),

    PurpleEntryArray(Vec<PurpleEntry>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleFF {
    DefFClass(DefFClass),

    Enum(DefF),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledGrp2 {
    Enum(PurpleGrp2),

    EnumArray(Vec<Grp2Element>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleBrief {
    BriefClass(BriefClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledProcStart {
    PurpleProcStart(PurpleProcStart),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousSyntax {
    FluffySyntax(FluffySyntax),

    PurpleSyntaxArray(Vec<PurpleSyntax>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DstUnion {
    SrcDst(SrcDst),

    SrcDstArray(Vec<SrcDst>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MnemUnion {
    MnemClass(MnemClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyFF {
    DefFClass(DefFClass),

    Enum(DefFEnum),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyGrp2 {
    Enum(Grp2Element),

    EnumArray(Vec<Grp2Element>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousSyntax {
    PurpleSyntaxArray(Vec<PurpleSyntax>),

    TentacledSyntax(TentacledSyntax),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledDst {
    PurpleDstArray(Vec<PurpleDst>),

    SrcDst(SrcDst),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriOpcdEntry {
    StickyEntry(StickyEntry),

    TentacledEntryArray(Vec<TentacledEntry>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoGrp2 {
    Enum(FluffyGrp2),

    EnumArray(Vec<Grp2Element>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyBrief {
    String(String),

    StringArray(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecOpcdUnion {
    SecOpcdClass(SecOpcdClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningSyntax {
    IndigoSyntax(IndigoSyntax),

    StickySyntaxArray(Vec<StickySyntax>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyDst {
    FluffyDst(FluffyDst),

    SrcDstArray(Vec<SrcDst>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleSrc {
    SrcDst(SrcDst),

    SrcDstArray(Vec<SrcDst>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffySrc {
    SrcClass(SrcClass),

    SrcDstArray(Vec<SrcDst>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyProcStart {
    FluffyProcStart(FluffyProcStart),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaSyntax {
    FluffySyntax(FluffySyntax),

    IndecentSyntaxArray(Vec<IndecentSyntax>),
}

#[derive(Serialize, Deserialize)]
pub enum PurpleAttr {
    #[serde(rename = "acc")]
    Acc,

    #[serde(rename = "delaysint")]
    Delaysint,

    #[serde(rename = "invd")]
    Invd,

    #[serde(rename = "nop")]
    Nop,

    #[serde(rename = "undef")]
    Undef,
}

#[derive(Serialize, Deserialize)]
pub enum Lock {
    #[serde(rename = "yes")]
    Yes,
}

#[derive(Serialize, Deserialize)]
pub enum DefF {
    #[serde(rename = "ac")]
    Ac,

    #[serde(rename = "c")]
    C,

    #[serde(rename = "i")]
    I,

    #[serde(rename = "oc")]
    Oc,

    #[serde(rename = "oszap")]
    Oszap,

    #[serde(rename = "oszapc")]
    Oszapc,

    #[serde(rename = "oszpc")]
    Oszpc,

    #[serde(rename = "ozpc")]
    Ozpc,

    #[serde(rename = "szapc")]
    Szapc,

    #[serde(rename = "szp")]
    Szp,

    #[serde(rename = "z")]
    Z,

    #[serde(rename = "zpc")]
    Zpc,
}

#[derive(Serialize, Deserialize)]
pub enum Doc {
    #[serde(rename = "m")]
    M,

    #[serde(rename = "u")]
    U,
}

#[derive(Serialize, Deserialize)]
pub enum Mod {
    #[serde(rename = "mem")]
    Mem,

    #[serde(rename = "nomem")]
    Nomem,
}

#[derive(Serialize, Deserialize)]
pub enum FVals {
    #[serde(rename = "i")]
    I,

    #[serde(rename = "o")]
    O,

    #[serde(rename = "oc")]
    Oc,
}

#[derive(Serialize, Deserialize)]
pub enum Fpop {
    #[serde(rename = "once")]
    Once,

    #[serde(rename = "twice")]
    Twice,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleGrp1 {
    #[serde(rename = "cachect")]
    Cachect,

    #[serde(rename = "gen")]
    Gen,

    #[serde(rename = "obsol")]
    Obsol,

    #[serde(rename = "prefix")]
    Prefix,

    #[serde(rename = "simdint")]
    Simdint,

    #[serde(rename = "system")]
    System,

    #[serde(rename = "x87fpu")]
    X87Fpu,
}

#[derive(Serialize, Deserialize)]
pub enum Grp2Element {
    #[serde(rename = "arith")]
    Arith,

    #[serde(rename = "branch")]
    Branch,

    #[serde(rename = "break")]
    Break,

    #[serde(rename = "datamov")]
    Datamov,

    #[serde(rename = "flgctrl")]
    Flgctrl,

    #[serde(rename = "inout")]
    Inout,

    #[serde(rename = "logical")]
    Logical,

    #[serde(rename = "segreg")]
    Segreg,

    #[serde(rename = "stack")]
    Stack,

    #[serde(rename = "string")]
    String,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleGrp2 {
    #[serde(rename = "arith")]
    Arith,

    #[serde(rename = "bit")]
    Bit,

    #[serde(rename = "branch")]
    Branch,

    #[serde(rename = "compar")]
    Compar,

    #[serde(rename = "control")]
    Control,

    #[serde(rename = "conver")]
    Conver,

    #[serde(rename = "datamov")]
    Datamov,

    #[serde(rename = "ldconst")]
    Ldconst,

    #[serde(rename = "logical")]
    Logical,

    #[serde(rename = "segreg")]
    Segreg,

    #[serde(rename = "shftrot")]
    Shftrot,

    #[serde(rename = "shunpck")]
    Shunpck,

    #[serde(rename = "stack")]
    Stack,

    #[serde(rename = "string")]
    String,

    #[serde(rename = "trans")]
    Trans,

    #[serde(rename = "x87fpu")]
    X87Fpu,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleGrp3 {
    #[serde(rename = "binary")]
    Binary,

    #[serde(rename = "cond")]
    Cond,

    #[serde(rename = "control")]
    Control,

    #[serde(rename = "decimal")]
    Decimal,
}

#[derive(Serialize, Deserialize)]
pub enum InstrExt {
    #[serde(rename = "mmx")]
    Mmx,

    #[serde(rename = "smx")]
    Smx,

    #[serde(rename = "sse1")]
    Sse1,

    #[serde(rename = "sse2")]
    Sse2,

    #[serde(rename = "sse3")]
    Sse3,

    #[serde(rename = "sse41")]
    Sse41,

    #[serde(rename = "sse42")]
    Sse42,

    #[serde(rename = "ssse3")]
    Ssse3,

    #[serde(rename = "vmx")]
    Vmx,
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "e")]
    E,

    #[serde(rename = "p")]
    P,
}

#[derive(Serialize, Deserialize)]
pub enum Pref {
    #[serde(rename = "F3")]
    F3,

    #[serde(rename = "9B")]
    The9B,
}

#[derive(Serialize, Deserialize)]
pub enum Post {
    #[serde(rename = "no")]
    No,
}

#[derive(Serialize, Deserialize)]
pub enum RingRef {
    #[serde(rename = "rflags_iopl")]
    RflagsIopl,
}

#[derive(Serialize, Deserialize)]
pub enum A {
    #[serde(rename = "A")]
    A,

    #[serde(rename = "C")]
    C,

    #[serde(rename = "D")]
    D,

    #[serde(rename = "E")]
    E,

    #[serde(rename = "ES")]
    Es,

    #[serde(rename = "EST")]
    Est,

    #[serde(rename = "G")]
    G,

    #[serde(rename = "H")]
    H,

    #[serde(rename = "I")]
    I,

    #[serde(rename = "J")]
    J,

    #[serde(rename = "M")]
    M,

    #[serde(rename = "N")]
    N,

    #[serde(rename = "O")]
    O,

    #[serde(rename = "P")]
    P,

    #[serde(rename = "Q")]
    Q,

    #[serde(rename = "R")]
    R,

    #[serde(rename = "S")]
    S,

    #[serde(rename = "T")]
    T,

    #[serde(rename = "U")]
    U,

    #[serde(rename = "V")]
    V,

    #[serde(rename = "W")]
    W,

    #[serde(rename = "Z")]
    Z,
}

#[derive(Serialize, Deserialize)]
pub enum Address {
    #[serde(rename = "BA")]
    Ba,

    #[serde(rename = "BB")]
    Bb,

    #[serde(rename = "BD")]
    Bd,

    #[serde(rename = "EST")]
    Est,

    #[serde(rename = "F")]
    F,

    #[serde(rename = "I")]
    I,

    #[serde(rename = "S2")]
    S2,

    #[serde(rename = "S30")]
    S30,

    #[serde(rename = "S33")]
    S33,

    #[serde(rename = "SC")]
    Sc,

    #[serde(rename = "X")]
    X,

    #[serde(rename = "Y")]
    Y,
}

#[derive(Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "b")]
    B,

    #[serde(rename = "d")]
    D,

    #[serde(rename = "da")]
    Da,

    #[serde(rename = "do")]
    Do,

    #[serde(rename = "dq")]
    Dq,

    #[serde(rename = "dqa")]
    Dqa,

    #[serde(rename = "dqp")]
    Dqp,

    #[serde(rename = "ps")]
    Ps,

    #[serde(rename = "q")]
    Q,

    #[serde(rename = "qa")]
    Qa,

    #[serde(rename = "qp")]
    Qp,

    #[serde(rename = "qs")]
    Qs,

    #[serde(rename = "v")]
    V,

    #[serde(rename = "va")]
    Va,

    #[serde(rename = "vds")]
    Vds,

    #[serde(rename = "vq")]
    Vq,

    #[serde(rename = "vqp")]
    Vqp,

    #[serde(rename = "w")]
    W,

    #[serde(rename = "wa")]
    Wa,

    #[serde(rename = "wo")]
    Wo,

    #[serde(rename = "ws")]
    Ws,
}

#[derive(Serialize, Deserialize)]
pub enum Group {
    #[serde(rename = "ctrl")]
    Ctrl,

    #[serde(rename = "debug")]
    Debug,

    #[serde(rename = "gen")]
    Gen,

    #[serde(rename = "mmx")]
    Mmx,

    #[serde(rename = "msr")]
    Msr,

    #[serde(rename = "seg")]
    Seg,

    #[serde(rename = "systabp")]
    Systabp,

    #[serde(rename = "x87fpu")]
    X87Fpu,

    #[serde(rename = "xcr")]
    Xcr,

    #[serde(rename = "xmm")]
    Xmm,
}

#[derive(Serialize, Deserialize)]
pub enum TestF {
    #[serde(rename = "a")]
    A,

    #[serde(rename = "ac")]
    Ac,

    #[serde(rename = "c")]
    C,

    #[serde(rename = "d")]
    D,

    #[serde(rename = "p")]
    P,

    #[serde(rename = "z")]
    Z,
}

#[derive(Serialize, Deserialize)]
pub enum UndefF {
    #[serde(rename = "a")]
    A,

    #[serde(rename = "o")]
    O,

    #[serde(rename = "oa")]
    Oa,

    #[serde(rename = "oac")]
    Oac,

    #[serde(rename = "oszapc")]
    Oszapc,

    #[serde(rename = "oszp")]
    Oszp,

    #[serde(rename = "szap")]
    Szap,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyAttr {
    #[serde(rename = "acc")]
    Acc,

    #[serde(rename = "delaysint_cond")]
    DelaysintCond,
}

#[derive(Serialize, Deserialize)]
pub enum DefFEnum {
    #[serde(rename = "c")]
    C,

    #[serde(rename = "d")]
    D,

    #[serde(rename = "i")]
    I,

    #[serde(rename = "oc")]
    Oc,

    #[serde(rename = "oszapc")]
    Oszapc,

    #[serde(rename = "oszpc")]
    Oszpc,

    #[serde(rename = "szapc")]
    Szapc,
}

#[derive(Serialize, Deserialize)]
pub enum TentacledAttr {
    #[serde(rename = "serial")]
    Serial,
}

#[derive(Serialize, Deserialize)]
pub enum DocRef {
    #[serde(rename = "gen_note_CMPXCHG8B_CMPXCHG16B_0FC7_1")]
    GenNoteCmpxchg8BCmpxchg16B0Fc71,

    #[serde(rename = "gen_note_hintable_nop_0F18_0F1F")]
    GenNoteHintableNop0F180F1F,

    #[serde(rename = "gen_note_MASKMOVQ_0FF7")]
    GenNoteMaskmovq0Ff7,

    #[serde(rename = "gen_note_SMSW_0F01_4")]
    GenNoteSmsw0F014,

    #[serde(rename = "gen_note_SSE4_amd")]
    GenNoteSse4Amd,

    #[serde(rename = "gen_note_SYSENTER_0F34")]
    GenNoteSysenter0F34,

    #[serde(rename = "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26")]
    GenNoteUMovCrDrTr0F200F210F220F230F240F26,

    #[serde(rename = "gen_note_VMX_vs_SVM")]
    GenNoteVmxVsSvm,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyGrp1 {
    #[serde(rename = "arith")]
    Arith,

    #[serde(rename = "cachect")]
    Cachect,

    #[serde(rename = "compar")]
    Compar,

    #[serde(rename = "conver")]
    Conver,

    #[serde(rename = "datamov")]
    Datamov,

    #[serde(rename = "fetch")]
    Fetch,

    #[serde(rename = "gen")]
    Gen,

    #[serde(rename = "logical")]
    Logical,

    #[serde(rename = "mxcsrsm")]
    Mxcsrsm,

    #[serde(rename = "order")]
    Order,

    #[serde(rename = "pcksclr")]
    Pcksclr,

    #[serde(rename = "pcksp")]
    Pcksp,

    #[serde(rename = "shift")]
    Shift,

    #[serde(rename = "simdfp")]
    Simdfp,

    #[serde(rename = "simdint")]
    Simdint,

    #[serde(rename = "sm")]
    Sm,

    #[serde(rename = "strtxt")]
    Strtxt,

    #[serde(rename = "sync")]
    Sync,

    #[serde(rename = "system")]
    System,

    #[serde(rename = "unpack")]
    Unpack,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyGrp2 {
    #[serde(rename = "arith")]
    Arith,

    #[serde(rename = "bit")]
    Bit,

    #[serde(rename = "branch")]
    Branch,

    #[serde(rename = "compar")]
    Compar,

    #[serde(rename = "control")]
    Control,

    #[serde(rename = "conver")]
    Conver,

    #[serde(rename = "datamov")]
    Datamov,

    #[serde(rename = "logical")]
    Logical,

    #[serde(rename = "shift")]
    Shift,

    #[serde(rename = "shunpck")]
    Shunpck,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyGrp3 {
    #[serde(rename = "binary")]
    Binary,

    #[serde(rename = "cond")]
    Cond,

    #[serde(rename = "trans")]
    Trans,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleT {
    #[serde(rename = "b")]
    B,

    #[serde(rename = "d")]
    D,

    #[serde(rename = "dq")]
    Dq,

    #[serde(rename = "dqp")]
    Dqp,

    #[serde(rename = "pd")]
    Pd,

    #[serde(rename = "pi")]
    Pi,

    #[serde(rename = "ps")]
    Ps,

    #[serde(rename = "psq")]
    Psq,

    #[serde(rename = "q")]
    Q,

    #[serde(rename = "s")]
    S,

    #[serde(rename = "sd")]
    Sd,

    #[serde(rename = "ss")]
    Ss,

    #[serde(rename = "stx")]
    Stx,

    #[serde(rename = "v")]
    V,

    #[serde(rename = "vqp")]
    Vqp,

    #[serde(rename = "w")]
    W,
}

#[derive(Serialize, Deserialize)]
pub enum Doc64Ref {
    #[serde(rename = "gen_note_BSF_0FBC_BSR_0FBD")]
    GenNoteBsf0FbcBsr0Fbd,

    #[serde(rename = "gen_note_CMOVcc_0F40-0F4F")]
    GenNoteCmoVcc0F400F4F,

    #[serde(rename = "gen_note_LSS_0FB2_LFS_0FB4_LGS_0FB5")]
    GenNoteLss0Fb2Lfs0Fb4Lgs0Fb5,

    #[serde(rename = "gen_note_short_near_jmp")]
    GenNoteShortNearJmp,

    #[serde(rename = "gen_note_SYSEXIT_0F35")]
    GenNoteSysexit0F35,
}

pub static JSON_DATA: &'static str = r###"
{
    "version": "1.11",
    "one-byte": [
      {
        "value": "00",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "ADD",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add"
          }
        }
      },
      {
        "value": "01",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "ADD",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add"
          }
        }
      },
      {
        "value": "02",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "ADD",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add"
          }
        }
      },
      {
        "value": "03",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "ADD",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add"
          }
        }
      },
      {
        "value": "04",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "ADD",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add"
          }
        }
      },
      {
        "value": "05",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "ADD",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add"
          }
        }
      },
      {
        "value": "06",
        "entry": [
          {
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "nr": "0",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "#text": "ES"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "07",
        "entry": [
          {
            "syntax": {
              "mnem": "POP",
              "dst": {
                "nr": "0",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "depend": "no",
                "#text": "ES"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "08",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "OR",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Inclusive OR"
          }
        }
      },
      {
        "value": "09",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "OR",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Inclusive OR"
          }
        }
      },
      {
        "value": "0A",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "OR",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Inclusive OR"
          }
        }
      },
      {
        "value": "0B",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "OR",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Inclusive OR"
          }
        }
      },
      {
        "value": "0C",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "OR",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Inclusive OR"
          }
        }
      },
      {
        "value": "0D",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "OR",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Inclusive OR"
          }
        }
      },
      {
        "value": "0E",
        "entry": [
          {
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "nr": "1",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "#text": "CS"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "0F",
        "entry": [
          {
            "doc1632_ref": "gen_note_opcd_POP_CS_0F",
            "proc_start": {
              "post": "no",
              "#text": "00"
            },
            "proc_end": "00",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "nr": "1",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "#text": "CS"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          },
          {
            "attr": "invd",
            "proc_start": {
              "post": "no",
              "#text": "01"
            },
            "proc_end": "01",
            "syntax": []
          },
          {
            "ref": "two-byte",
            "proc_start": "02",
            "syntax": []
          }
        ]
      },
      {
        "value": "10",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "ADC",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add with Carry"
          }
        }
      },
      {
        "value": "11",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "ADC",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add with Carry"
          }
        }
      },
      {
        "value": "12",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "ADC",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add with Carry"
          }
        }
      },
      {
        "value": "13",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "ADC",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add with Carry"
          }
        }
      },
      {
        "value": "14",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "ADC",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add with Carry"
          }
        }
      },
      {
        "value": "15",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "ADC",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Add with Carry"
          }
        }
      },
      {
        "value": "16",
        "entry": [
          {
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "nr": "2",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "#text": "SS"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "17",
        "entry": [
          {
            "attr": "delaysint",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "nr": "2",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "depend": "no",
                "#text": "SS"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "18",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "SBB",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Integer Subtraction with Borrow"
          }
        }
      },
      {
        "value": "19",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "SBB",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Integer Subtraction with Borrow"
          }
        }
      },
      {
        "value": "1A",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "SBB",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Integer Subtraction with Borrow"
          }
        }
      },
      {
        "value": "1B",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "SBB",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Integer Subtraction with Borrow"
          }
        }
      },
      {
        "value": "1C",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "SBB",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Integer Subtraction with Borrow"
          }
        }
      },
      {
        "value": "1D",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "SBB",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "test_f": "c",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Integer Subtraction with Borrow"
          }
        }
      },
      {
        "value": "1E",
        "entry": [
          {
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "nr": "3",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "#text": "DS"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "1F",
        "entry": [
          {
            "syntax": {
              "mnem": "POP",
              "dst": {
                "nr": "3",
                "group": "seg",
                "type": "w",
                "address": "S2",
                "depend": "no",
                "#text": "DS"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "20",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "AND",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical AND"
          }
        }
      },
      {
        "value": "21",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "AND",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical AND"
          }
        }
      },
      {
        "value": "22",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "AND",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical AND"
          }
        }
      },
      {
        "value": "23",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "AND",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical AND"
          }
        }
      },
      {
        "value": "24",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "AND",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical AND"
          }
        }
      },
      {
        "value": "25",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "AND",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical AND"
          }
        }
      },
      {
        "value": "26",
        "entry": [
          {
            "syntax": {
              "mnem": "ES",
              "src": {
                "nr": "0",
                "group": "seg",
                "type": "w",
                "displayed": "no",
                "#text": "ES"
              }
            },
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "ES segment override prefix"
            }
          },
          {
            "attr": "undef",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "(use with any branch instruction is reserved)"
            }
          },
          {
            "attr": null,
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "Null Prefix in 64-bit Mode"
            }
          }
        ]
      },
      {
        "value": "27",
        "entry": [
          {
            "syntax": {
              "mnem": "DAA",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "test_f": "ac",
            "modif_f": "oszapc",
            "def_f": "szapc",
            "undef_f": "o",
            "note": {
              "brief": "Decimal Adjust AL after Addition"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "28",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "SUB",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Subtract"
          }
        }
      },
      {
        "value": "29",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "SUB",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Subtract"
          }
        }
      },
      {
        "value": "2A",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "SUB",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Subtract"
          }
        }
      },
      {
        "value": "2B",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "SUB",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Subtract"
          }
        }
      },
      {
        "value": "2C",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "SUB",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Subtract"
          }
        }
      },
      {
        "value": "2D",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "SUB",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Subtract"
          }
        }
      },
      {
        "value": "2E",
        "entry": [
          {
            "syntax": {
              "mnem": "CS",
              "src": {
                "nr": "1",
                "group": "seg",
                "type": "w",
                "displayed": "no",
                "#text": "CS"
              }
            },
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "CS segment override prefix"
            }
          },
          {
            "doc1632_ref": "gen_note_branch_prefixes",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "NTAKEN"
              }
            },
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Branch not taken prefix (only with Jcc instructions)"
            }
          },
          {
            "attr": null,
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "Null Prefix in 64-bit Mode"
            }
          }
        ]
      },
      {
        "value": "2F",
        "entry": [
          {
            "syntax": {
              "mnem": "DAS",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "test_f": "ac",
            "modif_f": "oszapc",
            "def_f": "szapc",
            "undef_f": "o",
            "note": {
              "brief": "Decimal Adjust AL after Subtraction"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "30",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "XOR",
            "dst": {
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Exclusive OR"
          }
        }
      },
      {
        "value": "31",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "XOR",
            "dst": {
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Exclusive OR"
          }
        }
      },
      {
        "value": "32",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "XOR",
            "dst": {
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Exclusive OR"
          }
        }
      },
      {
        "value": "33",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "XOR",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Exclusive OR"
          }
        }
      },
      {
        "value": "34",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "XOR",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Exclusive OR"
          }
        }
      },
      {
        "value": "35",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "XOR",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Exclusive OR"
          }
        }
      },
      {
        "value": "36",
        "entry": [
          {
            "syntax": {
              "mnem": "SS",
              "src": {
                "nr": "2",
                "group": "seg",
                "type": "w",
                "displayed": "no",
                "#text": "SS"
              }
            },
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "SS segment override prefix"
            }
          },
          {
            "attr": "undef",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "(use with any branch instruction is reserved)"
            }
          },
          {
            "attr": null,
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "Null Prefix in 64-bit Mode"
            }
          }
        ]
      },
      {
        "value": "37",
        "entry": [
          {
            "syntax": {
              "mnem": "AAA",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AH"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "test_f": "a",
            "modif_f": "oszapc",
            "def_f": "ac",
            "undef_f": "oszp",
            "note": {
              "brief": "ASCII Adjust After Addition"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "38",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "CMP",
            "src": [
              {
                "a": "E",
                "t": "b"
              },
              {
                "a": "G",
                "t": "b"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare Two Operands"
          }
        }
      },
      {
        "value": "39",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "CMP",
            "src": [
              {
                "a": "E",
                "t": "vqp"
              },
              {
                "a": "G",
                "t": "vqp"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare Two Operands"
          }
        }
      },
      {
        "value": "3A",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "CMP",
            "src": [
              {
                "a": "G",
                "t": "b"
              },
              {
                "a": "E",
                "t": "b"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare Two Operands"
          }
        }
      },
      {
        "value": "3B",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "CMP",
            "src": [
              {
                "a": "G",
                "t": "vqp"
              },
              {
                "a": "E",
                "t": "vqp"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare Two Operands"
          }
        }
      },
      {
        "value": "3C",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "CMP",
            "src": [
              {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "#text": "AL"
              },
              {
                "a": "I",
                "t": "b"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare Two Operands"
          }
        }
      },
      {
        "value": "3D",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "CMP",
            "src": [
              {
                "nr": "0",
                "group": "gen",
                "type": "vqp",
                "#text": "rAX"
              },
              {
                "a": "I",
                "t": "vds"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare Two Operands"
          }
        }
      },
      {
        "value": "3E",
        "entry": [
          {
            "syntax": {
              "mnem": "DS",
              "src": {
                "nr": "3",
                "group": "seg",
                "type": "w",
                "displayed": "no",
                "#text": "DS"
              }
            },
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "DS segment override prefix"
            }
          },
          {
            "doc1632_ref": "gen_note_branch_prefixes",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "TAKEN"
              }
            },
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Branch taken prefix (only with Jcc instructions)"
            }
          },
          {
            "attr": null,
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "Null Prefix in 64-bit Mode"
            }
          }
        ]
      },
      {
        "value": "3F",
        "entry": [
          {
            "syntax": {
              "mnem": "AAS",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AH"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "test_f": "a",
            "modif_f": "oszapc",
            "def_f": "ac",
            "undef_f": "oszp",
            "note": {
              "brief": "ASCII Adjust AL After Subtraction"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "40",
        "entry": [
          {
            "syntax": {
              "mnem": "INC",
              "dst": {
                "a": "Z",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszap",
            "def_f": "oszap",
            "note": {
              "brief": "Increment by 1"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "REX"
            },
            "grp1": "prefix",
            "note": {
              "brief": "Access to new 8-bit registers"
            }
          }
        ]
      },
      {
        "value": "41",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.B"
          },
          "grp1": "prefix",
          "note": {
            "brief": "Extension of r/m field, base field, or opcode reg field"
          }
        }
      },
      {
        "value": "42",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.X"
          },
          "grp1": "prefix",
          "note": {
            "brief": "Extension of SIB index field"
          }
        }
      },
      {
        "value": "43",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.XB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.X and REX.B combination"
          }
        }
      },
      {
        "value": "44",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.R"
          },
          "grp1": "prefix",
          "note": {
            "brief": "Extension of ModR/M reg field"
          }
        }
      },
      {
        "value": "45",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.RB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.R and REX.B combination"
          }
        }
      },
      {
        "value": "46",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.RX"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.R and REX.X combination"
          }
        }
      },
      {
        "value": "47",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.RXB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.R, REX.X and REX.B combination"
          }
        }
      },
      {
        "value": "48",
        "entry": [
          {
            "syntax": {
              "mnem": "DEC",
              "dst": {
                "a": "Z",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszap",
            "def_f": "oszap",
            "note": {
              "brief": "Decrement by 1"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "REX.W"
            },
            "grp1": "prefix",
            "note": {
              "brief": "64 Bit Operand Size"
            }
          }
        ]
      },
      {
        "value": "49",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W and REX.B combination"
          }
        }
      },
      {
        "value": "4A",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WX"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W and REX.X combination"
          }
        }
      },
      {
        "value": "4B",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WXB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W, REX.X and REX.B combination"
          }
        }
      },
      {
        "value": "4C",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WR"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W and REX.R combination"
          }
        }
      },
      {
        "value": "4D",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WRB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W, REX.R and REX.B combination"
          }
        }
      },
      {
        "value": "4E",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WRX"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W, REX.R and REX.X combination"
          }
        }
      },
      {
        "value": "4F",
        "entry": {
          "mode": "e",
          "proc_start": "10",
          "syntax": {
            "mnem": "REX.WRXB"
          },
          "grp1": "prefix",
          "note": {
            "brief": "REX.W, REX.R, REX.X and REX.B combination"
          }
        }
      },
      {
        "value": "50",
        "entry": [
          {
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "Z",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "Z",
                "t": "vq"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          }
        ]
      },
      {
        "value": "58",
        "entry": [
          {
            "syntax": {
              "mnem": "POP",
              "dst": {
                "depend": "no",
                "a": "Z",
                "t": "v"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "depend": "no",
                "a": "Z",
                "t": "vq"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          }
        ]
      },
      {
        "value": "60",
        "entry": [
          {
            "proc_start": "01",
            "syntax": {
              "mnem": "PUSHA",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "CX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "DX"
                },
                {
                  "nr": "3",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "BX"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "SP"
                },
                {
                  "nr": "5",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "BP"
                },
                {
                  "nr": "6",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "SI"
                },
                {
                  "nr": "7",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "DI"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Push All General-Purpose Registers"
            }
          },
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "PUSHAD",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EAX"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "ECX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EDX"
                },
                {
                  "nr": "3",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EBX"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "ESP"
                },
                {
                  "nr": "5",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EBP"
                },
                {
                  "nr": "6",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "ESI"
                },
                {
                  "nr": "7",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EDI"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Push All General-Purpose Registers"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "61",
        "entry": [
          {
            "proc_start": "01",
            "syntax": {
              "mnem": "POPA",
              "dst": [
                {
                  "nr": "7",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "DI"
                },
                {
                  "nr": "6",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "SI"
                },
                {
                  "nr": "5",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "BP"
                },
                {
                  "nr": "3",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "BX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "DX"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "CX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                }
              ],
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Pop All General-Purpose Registers"
            }
          },
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "POPAD",
              "dst": [
                {
                  "nr": "7",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EDI"
                },
                {
                  "nr": "6",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "ESI"
                },
                {
                  "nr": "5",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EBP"
                },
                {
                  "nr": "3",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EBX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EDX"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "ECX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EAX"
                }
              ],
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Pop All General-Purpose Registers"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "62",
        "entry": [
          {
            "direction": "1",
            "r": "yes",
            "ring": "f",
            "proc_start": "01",
            "syntax": {
              "mnem": "BOUND",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": [
                {
                  "a": "G",
                  "t": "v"
                },
                {
                  "a": "M",
                  "t": "a"
                },
                {
                  "type": "v",
                  "address": "F",
                  "displayed": "no",
                  "#text": "eFlags"
                }
              ]
            },
            "grp1": "gen",
            "grp2": [
              "break",
              "stack"
            ],
            "modif_f": {
              "cond": "yes",
              "#text": "i"
            },
            "def_f": {
              "cond": "yes",
              "#text": "i"
            },
            "f_vals": "i",
            "note": {
              "brief": "Check Array Index Against Bounds"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "63",
        "entry": [
          {
            "r": "yes",
            "proc_start": "02",
            "syntax": {
              "mnem": "ARPL",
              "src": [
                {
                  "a": "E",
                  "t": "w"
                },
                {
                  "a": "G",
                  "t": "w"
                }
              ]
            },
            "grp1": "system",
            "modif_f": "z",
            "def_f": "z",
            "note": {
              "brief": "Adjust RPL Field of Segment Selector"
            }
          },
          {
            "direction": "1",
            "r": "yes",
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "MOVSXD",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "dqp"
              },
              "src": {
                "a": "E",
                "t": "d"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Move with Sign-Extension"
            }
          }
        ]
      },
      {
        "value": "64",
        "entry": [
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "FS",
              "src": {
                "nr": "4",
                "group": "seg",
                "type": "w",
                "displayed": "no",
                "#text": "FS"
              }
            },
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "FS segment override prefix"
            }
          },
          {
            "attr": "undef",
            "is_undoc": "yes",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "(only with Jcc instructions)"
            }
          },
          {
            "doc": "u",
            "is_doc": "yes",
            "doc1632_ref": "gen_note_branch_prefixes",
            "particular": "yes",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "ALTER"
              }
            },
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Alternating branch prefix (only with Jcc instructions)"
            }
          }
        ]
      },
      {
        "value": "65",
        "entry": [
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "GS",
              "src": {
                "nr": "5",
                "group": "seg",
                "type": "w",
                "displayed": "no",
                "#text": "GS"
              }
            },
            "grp1": "prefix",
            "grp2": "segreg",
            "note": {
              "brief": "GS segment override prefix"
            }
          },
          {
            "attr": "undef",
            "proc_start": {
              "post": "no",
              "#text": "10"
            },
            "proc_end": "10",
            "syntax": [],
            "grp1": "prefix",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "(only with Jcc instructions)"
            }
          }
        ]
      },
      {
        "value": "66",
        "entry": [
          {
            "syntax": [],
            "grp1": "prefix",
            "note": {
              "brief": "Operand-size override prefix"
            }
          },
          {
            "doc": "m",
            "proc_start": "10",
            "syntax": [],
            "instr_ext": "sse2",
            "grp1": "prefix",
            "note": {
              "brief": "Precision-size override prefix"
            }
          }
        ]
      },
      {
        "value": "67",
        "entry": {
          "syntax": [],
          "grp1": "prefix",
          "note": {
            "brief": "Address-size override prefix"
          }
        }
      },
      {
        "value": "68",
        "entry": {
          "proc_start": "01",
          "syntax": {
            "mnem": "PUSH",
            "dst": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            },
            "src": {
              "a": "I",
              "t": "vs"
            }
          },
          "grp1": "gen",
          "grp2": "stack",
          "note": {
            "brief": "Push Word, Doubleword or Quadword Onto the Stack"
          }
        }
      },
      {
        "value": "69",
        "entry": {
          "r": "yes",
          "proc_start": "01",
          "syntax": {
            "mnem": "IMUL",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": [
              {
                "a": "E",
                "t": "vqp"
              },
              {
                "a": "I",
                "t": "vds"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oc",
          "undef_f": "szap",
          "note": {
            "brief": "Signed Multiply"
          }
        }
      },
      {
        "value": "6A",
        "entry": {
          "sign-ext": "1",
          "proc_start": "01",
          "syntax": {
            "mnem": "PUSH",
            "dst": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            },
            "src": {
              "a": "I",
              "t": "bss"
            }
          },
          "grp1": "gen",
          "grp2": "stack",
          "note": {
            "brief": "Push Word, Doubleword or Quadword Onto the Stack"
          }
        }
      },
      {
        "value": "6B",
        "entry": {
          "sign-ext": "1",
          "r": "yes",
          "proc_start": "01",
          "syntax": {
            "mnem": "IMUL",
            "dst": {
              "a": "G",
              "t": "vqp"
            },
            "src": [
              {
                "a": "E",
                "t": "vqp"
              },
              {
                "a": "I",
                "t": "bs"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oc",
          "undef_f": "szap",
          "note": {
            "brief": "Signed Multiply"
          }
        }
      },
      {
        "value": "6C",
        "entry": {
          "op_size": "0",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "proc_start": "01",
          "syntax": [
            {
              "mnem": "INS",
              "dst": {
                "type": "b",
                "address": "Y",
                "depend": "no",
                "#text": "(ES:)[rDI]"
              },
              "src": {
                "nr": "2",
                "group": "gen",
                "type": "w",
                "#text": "DX"
              }
            },
            {
              "mnem": "INSB",
              "dst": {
                "type": "b",
                "address": "Y",
                "displayed": "no",
                "depend": "no",
                "#text": "(ES:)[rDI]"
              },
              "src": {
                "nr": "2",
                "group": "gen",
                "type": "w",
                "displayed": "no",
                "#text": "DX"
              }
            }
          ],
          "grp1": "gen",
          "grp2": [
            "inout",
            "string"
          ],
          "test_f": "d",
          "note": {
            "brief": "Input from Port to String"
          }
        }
      },
      {
        "value": "6D",
        "entry": [
          {
            "op_size": "1",
            "ring": "f",
            "ring_ref": "rflags_iopl",
            "proc_start": "01",
            "syntax": [
              {
                "mnem": "INS",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "depend": "no",
                  "#text": "ES:[DI]"
                },
                "src": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "#text": "DX"
                }
              },
              {
                "mnem": "INSW",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "ES:[DI]"
                },
                "src": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "#text": "DX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "inout",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Input from Port to String"
            }
          },
          {
            "op_size": "1",
            "ring": "f",
            "ring_ref": "rflags_iopl",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "INS",
                "dst": {
                  "type": "v",
                  "address": "Y",
                  "depend": "no",
                  "#text": "(ES:)[rDI]"
                },
                "src": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "#text": "DX"
                }
              },
              {
                "mnem": "INSD",
                "dst": {
                  "type": "do",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "(ES:)[rDI]"
                },
                "src": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "#text": "DX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "inout",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Input from Port to String"
            }
          }
        ]
      },
      {
        "value": "6E",
        "entry": {
          "op_size": "0",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "proc_start": "01",
          "syntax": [
            {
              "mnem": "OUTS",
              "dst": {
                "nr": "2",
                "group": "gen",
                "type": "w",
                "depend": "no",
                "#text": "DX"
              },
              "src": {
                "type": "b",
                "address": "X",
                "#text": "(DS):[rSI]"
              }
            },
            {
              "mnem": "OUTSB",
              "dst": {
                "nr": "2",
                "group": "gen",
                "type": "w",
                "displayed": "no",
                "depend": "no",
                "#text": "DX"
              },
              "src": {
                "type": "b",
                "address": "X",
                "displayed": "no",
                "#text": "(DS):[rSI]"
              }
            }
          ],
          "grp1": "gen",
          "grp2": [
            "inout",
            "string"
          ],
          "test_f": "d",
          "note": {
            "brief": "Output String to Port"
          }
        }
      },
      {
        "value": "6F",
        "entry": [
          {
            "op_size": "1",
            "ring": "f",
            "ring_ref": "rflags_iopl",
            "proc_start": "01",
            "syntax": [
              {
                "mnem": "OUTS",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "depend": "no",
                  "#text": "DX"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "#text": "DS:[SI]"
                }
              },
              {
                "mnem": "OUTSW",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "DX"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "displayed": "no",
                  "#text": "DS:[SI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "inout",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Output String to Port"
            }
          },
          {
            "op_size": "1",
            "ring": "f",
            "ring_ref": "rflags_iopl",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "OUTS",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "depend": "no",
                  "#text": "DX"
                },
                "src": {
                  "type": "v",
                  "address": "X",
                  "#text": "(DS:)[rSI]"
                }
              },
              {
                "mnem": "OUTSD",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "DX"
                },
                "src": {
                  "type": "do",
                  "address": "X",
                  "displayed": "no",
                  "#text": "(DS:)[rSI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "inout",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Output String to Port"
            }
          }
        ]
      },
      {
        "value": "70",
        "entry": {
          "tttn": "0000",
          "syntax": {
            "mnem": "JO",
            "src": {
              "a": "J",
              "t": "bs"
            }
          },
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "o",
          "note": {
            "brief": "Jump short if overflow (OF=1)"
          }
        }
      },
      {
        "value": "71",
        "entry": {
          "tttn": "0001",
          "syntax": {
            "mnem": "JNO",
            "src": {
              "a": "J",
              "t": "bs"
            }
          },
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "o",
          "note": {
            "brief": "Jump short if not overflow (OF=0)"
          }
        }
      },
      {
        "value": "72",
        "entry": {
          "tttn": "0010",
          "syntax": [
            {
              "mnem": "JB",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JNAE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JC",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "c",
          "note": {
            "brief": "Jump short if below/not above or equal/carry (CF=1)"
          }
        }
      },
      {
        "value": "73",
        "entry": {
          "tttn": "0011",
          "syntax": [
            {
              "mnem": "JNB",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JAE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JNC",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "c",
          "note": {
            "brief": "Jump short if not below/above or equal/not carry (CF=0)"
          }
        }
      },
      {
        "value": "74",
        "entry": {
          "tttn": "0100",
          "syntax": [
            {
              "mnem": "JZ",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "z",
          "note": {
            "brief": "Jump short if zero/equal (ZF=0)"
          }
        }
      },
      {
        "value": "75",
        "entry": {
          "tttn": "0101",
          "syntax": [
            {
              "mnem": "JNZ",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JNE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "z",
          "note": {
            "brief": "Jump short if not zero/not equal (ZF=1)"
          }
        }
      },
      {
        "value": "76",
        "entry": {
          "tttn": "0110",
          "syntax": [
            {
              "mnem": "JBE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JNA",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "zc",
          "note": {
            "brief": "Jump short if below or equal/not above (CF=1 AND ZF=1)"
          }
        }
      },
      {
        "value": "77",
        "entry": {
          "tttn": "0111",
          "syntax": [
            {
              "mnem": "JNBE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JA",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "zc",
          "note": {
            "brief": "Jump short if not below or equal/above (CF=0 AND ZF=0)"
          }
        }
      },
      {
        "value": "78",
        "entry": {
          "tttn": "1000",
          "syntax": {
            "mnem": "JS",
            "src": {
              "a": "J",
              "t": "bs"
            }
          },
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "s",
          "note": {
            "brief": "Jump short if sign (SF=1)"
          }
        }
      },
      {
        "value": "79",
        "entry": {
          "tttn": "1001",
          "syntax": {
            "mnem": "JNS",
            "src": {
              "a": "J",
              "t": "bs"
            }
          },
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "s",
          "note": {
            "brief": "Jump short if not sign (SF=0)"
          }
        }
      },
      {
        "value": "7A",
        "entry": {
          "tttn": "1010",
          "syntax": [
            {
              "mnem": "JP",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JPE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "p",
          "note": {
            "brief": "Jump short if parity/parity even (PF=1)"
          }
        }
      },
      {
        "value": "7B",
        "entry": {
          "tttn": "1011",
          "syntax": [
            {
              "mnem": "JNP",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JPO",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "p",
          "note": {
            "brief": "Jump short if not parity/parity odd"
          }
        }
      },
      {
        "value": "7C",
        "entry": {
          "tttn": "1100",
          "syntax": [
            {
              "mnem": "JL",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JNGE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "os",
          "note": {
            "brief": "Jump short if less/not greater (SF!=OF)"
          }
        }
      },
      {
        "value": "7D",
        "entry": {
          "tttn": "1101",
          "syntax": [
            {
              "mnem": "JNL",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JGE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "os",
          "note": {
            "brief": "Jump short if not less/greater or equal (SF=OF)"
          }
        }
      },
      {
        "value": "7E",
        "entry": {
          "tttn": "1110",
          "syntax": [
            {
              "mnem": "JLE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JNG",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "osz",
          "note": {
            "brief": "Jump short if less or equal/not greater ((ZF=1) OR (SF!=OF))"
          }
        }
      },
      {
        "value": "7F",
        "entry": {
          "tttn": "1111",
          "syntax": [
            {
              "mnem": "JNLE",
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            {
              "mnem": "JG",
              "src": {
                "a": "J",
                "t": "bs"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "branch",
          "grp3": "cond",
          "test_f": "osz",
          "note": {
            "brief": "Jump short if not less nor equal/greater ((ZF=0) AND (SF=OF))"
          }
        }
      },
      {
        "value": "80",
        "entry": [
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ADD",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add"
            }
          },
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "OR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Inclusive OR"
            }
          },
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "ADC",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add with Carry"
            }
          },
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "SBB",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Integer Subtraction with Borrow"
            }
          },
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "AND",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical AND"
            }
          },
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SUB",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "op_size": "0",
            "lock": "yes",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "XOR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Exclusive OR"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "CMP",
              "src": [
                {
                  "a": "E",
                  "t": "b"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare Two Operands"
            }
          }
        ]
      },
      {
        "value": "81",
        "entry": [
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ADD",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add"
            }
          },
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "OR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Inclusive OR"
            }
          },
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "ADC",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add with Carry"
            }
          },
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "SBB",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Integer Subtraction with Borrow"
            }
          },
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "AND",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical AND"
            }
          },
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SUB",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "XOR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Exclusive OR"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "CMP",
              "src": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "a": "I",
                  "t": "vds"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare Two Operands"
            }
          }
        ]
      },
      {
        "value": "82",
        "entry": [
          {
            "op_size": "0",
            "alias": "80_0",
            "lock": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ADD",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add"
            }
          },
          {
            "op_size": "0",
            "alias": "80_1",
            "lock": "yes",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "OR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Inclusive OR"
            }
          },
          {
            "op_size": "0",
            "alias": "80_2",
            "lock": "yes",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "ADC",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add with Carry"
            }
          },
          {
            "op_size": "0",
            "alias": "80_3",
            "lock": "yes",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "SBB",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Integer Subtraction with Borrow"
            }
          },
          {
            "op_size": "0",
            "alias": "80_4",
            "lock": "yes",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "AND",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical AND"
            }
          },
          {
            "op_size": "0",
            "alias": "80_5",
            "lock": "yes",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SUB",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "op_size": "0",
            "alias": "80_6",
            "lock": "yes",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "XOR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Exclusive OR"
            }
          },
          {
            "op_size": "0",
            "alias": "80_7",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "CMP",
              "src": [
                {
                  "a": "E",
                  "t": "b"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare Two Operands"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "83",
        "entry": [
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ADD",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "1",
            "proc_start": "03",
            "syntax": {
              "mnem": "OR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Inclusive OR"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "ADC",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Add with Carry"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "SBB",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Integer Subtraction with Borrow"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "4",
            "proc_start": "03",
            "syntax": {
              "mnem": "AND",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical AND"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SUB",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "lock": "yes",
            "opcd_ext": "6",
            "proc_start": "03",
            "syntax": {
              "mnem": "XOR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Exclusive OR"
            }
          },
          {
            "sign-ext": "1",
            "op_size": "1",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "CMP",
              "src": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "a": "I",
                  "t": "bs"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare Two Operands"
            }
          }
        ]
      },
      {
        "value": "84",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "TEST",
            "src": [
              {
                "a": "E",
                "t": "b"
              },
              {
                "a": "G",
                "t": "b"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Compare"
          }
        }
      },
      {
        "value": "85",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "TEST",
            "src": [
              {
                "a": "E",
                "t": "vqp"
              },
              {
                "a": "G",
                "t": "vqp"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "arith",
          "grp3": "binary",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Compare"
          }
        }
      },
      {
        "value": "86",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "XCHG",
            "dst": [
              {
                "a": "G",
                "t": "b"
              },
              {
                "a": "E",
                "t": "b"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Exchange Register/Memory with Register"
          }
        }
      },
      {
        "value": "87",
        "entry": {
          "direction": "1",
          "op_size": "1",
          "r": "yes",
          "lock": "yes",
          "syntax": {
            "mnem": "XCHG",
            "dst": [
              {
                "a": "G",
                "t": "vqp"
              },
              {
                "a": "E",
                "t": "vqp"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Exchange Register/Memory with Register"
          }
        }
      },
      {
        "value": "88",
        "entry": {
          "direction": "0",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "G",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "89",
        "entry": {
          "direction": "0",
          "op_size": "1",
          "r": "yes",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "G",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "8A",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "G",
              "t": "b"
            },
            "src": {
              "a": "E",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "8B",
        "entry": {
          "direction": "1",
          "op_size": "0",
          "r": "yes",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "a": "E",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "8C",
        "entry": {
          "direction": "0",
          "r": "yes",
          "syntax": [
            {
              "mod": "mem",
              "mnem": "MOV",
              "dst": {
                "depend": "no",
                "a": "M",
                "t": "w"
              },
              "src": {
                "a": "S",
                "t": "w"
              }
            },
            {
              "mod": "nomem",
              "mnem": "MOV",
              "dst": {
                "depend": "no",
                "a": "R",
                "t": "vqp"
              },
              "src": {
                "a": "S",
                "t": "w"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "8D",
        "entry": {
          "r": "yes",
          "syntax": {
            "mnem": "LEA",
            "dst": {
              "depend": "no",
              "a": "G",
              "t": "vqp"
            },
            "src": {
              "depend": "no",
              "a": "M"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Load Effective Address"
          }
        }
      },
      {
        "value": "8E",
        "entry": {
          "direction": "1",
          "r": "yes",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "S",
              "t": "w"
            },
            "src": {
              "a": "E",
              "t": "w"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "8F",
        "entry": [
          {
            "op_size": "1",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "depend": "no",
                "a": "E",
                "t": "v"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          },
          {
            "op_size": "1",
            "mode": "e",
            "opcd_ext": "0",
            "proc_start": "10",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "depend": "no",
                "a": "E",
                "t": "vq"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          }
        ]
      },
      {
        "value": "90",
        "entry": [
          {
            "attr": "acc",
            "syntax": {
              "mnem": "XCHG",
              "dst": [
                {
                  "a": "Z",
                  "t": "vqp"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "#text": "rAX"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "datamov",
            "note": {
              "brief": "Exchange Register/Memory with Register"
            }
          },
          {
            "doc_ref": "gen_note_90_NOP",
            "syntax": {
              "mnem": "NOP"
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "No Operation"
            }
          },
          {
            "attr": "nop",
            "doc_ref": "gen_note_plain_F390",
            "particular": "yes",
            "pref": "F3",
            "proc_end": "09",
            "syntax": {
              "mnem": "NOP"
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "No Operation"
            }
          },
          {
            "pref": "F3",
            "proc_start": "10",
            "syntax": {
              "mnem": "PAUSE"
            },
            "instr_ext": "sse2",
            "grp1": "cachect",
            "note": {
              "brief": "Spin Loop Hint"
            }
          }
        ]
      },
      {
        "value": "98",
        "entry": [
          {
            "syntax": {
              "mnem": "CBW",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "wo",
                "displayed": "no",
                "#text": "AX"
              },
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Convert Byte to Word"
            }
          },
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "CWDE",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "do",
                "displayed": "no",
                "#text": "EAX"
              },
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "w",
                "displayed": "no",
                "#text": "AX"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Convert Word to Doubleword"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "CBW",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                }
              },
              {
                "mnem": "CWDE",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EAX"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "#text": "AX"
                }
              },
              {
                "mnem": "CDQE",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "qp",
                  "displayed": "no",
                  "#text": "RAX"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "d",
                  "displayed": "no",
                  "#text": "EAX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Convert"
            }
          }
        ]
      },
      {
        "value": "99",
        "entry": [
          {
            "syntax": {
              "mnem": "CWD",
              "dst": {
                "nr": "2",
                "group": "gen",
                "type": "wo",
                "displayed": "no",
                "depend": "no",
                "#text": "DX"
              },
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "wo",
                "displayed": "no",
                "#text": "AX"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Convert Word to Doubleword"
            }
          },
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "CDQ",
              "dst": {
                "nr": "2",
                "group": "gen",
                "type": "do",
                "displayed": "no",
                "depend": "no",
                "#text": "EDX"
              },
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "do",
                "displayed": "no",
                "#text": "EAX"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Convert Doubleword to Quadword"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "CWD",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "DX"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                }
              },
              {
                "mnem": "CDQ",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "EDX"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EAX"
                }
              },
              {
                "mnem": "CQO",
                "dst": {
                  "nr": "2",
                  "group": "gen",
                  "type": "qp",
                  "displayed": "no",
                  "#text": "RDX"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "qp",
                  "displayed": "no",
                  "#text": "RAX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Convert"
            }
          }
        ]
      },
      {
        "value": "9A",
        "entry": [
          {
            "syntax": {
              "mnem": "CALLF",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "A",
                "t": "p"
              }
            },
            "grp1": "gen",
            "grp2": [
              "branch",
              "stack"
            ],
            "note": {
              "brief": "Call Procedure"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "9B",
        "entry": [
          {
            "syntax": [
              {
                "mnem": "FWAIT"
              },
              {
                "mnem": "WAIT"
              }
            ],
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Check pending unmasked floating-point exceptions"
            }
          },
          {
            "syntax": [],
            "grp1": "prefix",
            "grp2": "x87fpu",
            "grp3": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Wait Prefix"
            }
          }
        ]
      },
      {
        "value": "9C",
        "entry": [
          {
            "syntax": {
              "mnem": "PUSHF",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "type": "wo",
                "address": "F",
                "displayed": "no",
                "#text": "Flags"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "flgctrl"
            ],
            "note": {
              "brief": "Push FLAGS Register onto the Stack"
            }
          },
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "PUSHFD",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "type": "do",
                "address": "F",
                "displayed": "no",
                "#text": "EFlags"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "flgctrl"
            ],
            "note": {
              "brief": "Push eFLAGS Register onto the Stack"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "PUSHF",
                "dst": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                },
                "src": {
                  "type": "ws",
                  "address": "F",
                  "displayed": "no",
                  "#text": "Flags"
                }
              },
              {
                "mnem": "PUSHFQ",
                "dst": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                },
                "src": {
                  "type": "qs",
                  "address": "F",
                  "displayed": "no",
                  "#text": "RFlags"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "stack",
              "flgctrl"
            ],
            "note": {
              "brief": "Push rFLAGS Register onto the Stack"
            }
          }
        ]
      },
      {
        "value": "9D",
        "entry": [
          {
            "syntax": {
              "mnem": "POPF",
              "dst": {
                "type": "wo",
                "address": "F",
                "displayed": "no",
                "depend": "no",
                "#text": "Flags"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "flgctrl"
            ],
            "note": {
              "brief": "Pop Stack into FLAGS Register"
            }
          },
          {
            "proc_start": "03",
            "syntax": {
              "mnem": "POPFD",
              "dst": {
                "type": "do",
                "address": "F",
                "displayed": "no",
                "depend": "no",
                "#text": "EFlags"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "flgctrl"
            ],
            "note": {
              "brief": "Pop Stack into eFLAGS Register"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "POPF",
                "dst": {
                  "type": "ws",
                  "address": "F",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "Flags"
                },
                "src": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                }
              },
              {
                "mnem": "POPFQ",
                "dst": {
                  "type": "qs",
                  "address": "F",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "RFlags"
                },
                "src": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "stack",
              "flgctrl"
            ],
            "note": {
              "brief": "Pop Stack into rFLAGS Register"
            }
          }
        ]
      },
      {
        "value": "9E",
        "entry": {
          "doc64_ref": "gen_note_SAHF_9E_LAHF_9F",
          "syntax": {
            "mnem": "SAHF",
            "src": {
              "nr": "4",
              "group": "gen",
              "type": "b",
              "displayed": "no",
              "#text": "AH"
            }
          },
          "grp1": "gen",
          "grp2": [
            "datamov",
            "flgctrl"
          ],
          "modif_f": "szapc",
          "def_f": "szapc",
          "note": {
            "brief": "Store AH into Flags"
          }
        }
      },
      {
        "value": "9F",
        "entry": {
          "doc64_ref": "gen_note_SAHF_9E_LAHF_9F",
          "syntax": {
            "mnem": "LAHF",
            "dst": {
              "nr": "4",
              "group": "gen",
              "type": "b",
              "displayed": "no",
              "depend": "no",
              "#text": "AH"
            }
          },
          "grp1": "gen",
          "grp2": [
            "datamov",
            "flgctrl"
          ],
          "test_f": "szapc",
          "note": {
            "brief": "Load Status Flags into AH Register"
          }
        }
      },
      {
        "value": "A0",
        "entry": {
          "op_size": "0",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "depend": "no",
              "#text": "AL"
            },
            "src": {
              "a": "O",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "A1",
        "entry": {
          "op_size": "1",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "depend": "no",
              "#text": "rAX"
            },
            "src": {
              "a": "O",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "A2",
        "entry": {
          "op_size": "0",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "O",
              "t": "b"
            },
            "src": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "A3",
        "entry": {
          "op_size": "1",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "O",
              "t": "vqp"
            },
            "src": {
              "nr": "0",
              "group": "gen",
              "type": "vqp",
              "#text": "rAX"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "A4",
        "entry": {
          "op_size": "0",
          "syntax": [
            {
              "mnem": "MOVS",
              "dst": {
                "type": "b",
                "address": "Y",
                "depend": "no",
                "#text": "(ES:)[rDI]"
              },
              "src": {
                "type": "b",
                "address": "X",
                "#text": "(DS:)[rSI]"
              }
            },
            {
              "mnem": "MOVSB",
              "dst": {
                "type": "b",
                "address": "Y",
                "displayed": "no",
                "depend": "no",
                "#text": "(ES:)[rDI]"
              },
              "src": {
                "type": "b",
                "address": "X",
                "displayed": "no",
                "#text": "(DS:)[rSI]"
              }
            }
          ],
          "grp1": "gen",
          "grp2": [
            "datamov",
            "string"
          ],
          "test_f": "d",
          "note": {
            "brief": "Move Data from String to String"
          }
        }
      },
      {
        "value": "A5",
        "entry": [
          {
            "op_size": "1",
            "syntax": [
              {
                "mnem": "MOVS",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "depend": "no",
                  "#text": "ES:[DI]"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "#text": "DS:[SI]"
                }
              },
              {
                "mnem": "MOVSW",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "ES:[DI]"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "displayed": "no",
                  "#text": "DS:[SI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Move Data from String to String"
            }
          },
          {
            "op_size": "1",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "MOVS",
                "dst": {
                  "type": "v",
                  "address": "Y",
                  "depend": "no",
                  "#text": "(ES:)[rDI]"
                },
                "src": {
                  "type": "v",
                  "address": "X",
                  "#text": "(DS:)[rSI]"
                }
              },
              {
                "mnem": "MOVSD",
                "dst": {
                  "type": "do",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "(ES:)[rDI]"
                },
                "src": {
                  "type": "do",
                  "address": "X",
                  "displayed": "no",
                  "#text": "(DS:)[rSI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Move Data from String to String"
            }
          },
          {
            "op_size": "1",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "MOVS",
                "dst": {
                  "type": "vqp",
                  "address": "Y",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "type": "vqp",
                  "address": "X",
                  "#text": "[rSI]"
                }
              },
              {
                "mnem": "MOVSW",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "displayed": "no",
                  "#text": "[rSI]"
                }
              },
              {
                "mnem": "MOVSD",
                "dst": {
                  "type": "do",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "type": "do",
                  "address": "X",
                  "displayed": "no",
                  "#text": "[rSI]"
                }
              },
              {
                "mnem": "MOVSQ",
                "dst": {
                  "type": "qp",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "type": "qp",
                  "address": "X",
                  "displayed": "no",
                  "#text": "[rSI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Move Data from String to String"
            }
          }
        ]
      },
      {
        "value": "A6",
        "entry": {
          "op_size": "0",
          "syntax": [
            {
              "mnem": "CMPS",
              "src": [
                {
                  "type": "b",
                  "address": "Y",
                  "#text": "(ES:)[rDI]"
                },
                {
                  "type": "b",
                  "address": "X",
                  "#text": "(DS:)[rSI]"
                }
              ]
            },
            {
              "mnem": "CMPSB",
              "src": [
                {
                  "type": "b",
                  "address": "Y",
                  "displayed": "no",
                  "#text": "(ES:)[rDI]"
                },
                {
                  "type": "b",
                  "address": "X",
                  "displayed": "no",
                  "#text": "(DS:)[rSI]"
                }
              ]
            }
          ],
          "grp1": "gen",
          "grp2": [
            "arith",
            "string"
          ],
          "grp3": "binary",
          "test_f": "d",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Compare String Operands"
          }
        }
      },
      {
        "value": "A7",
        "entry": [
          {
            "op_size": "1",
            "syntax": [
              {
                "mnem": "CMPS",
                "src": [
                  {
                    "type": "wo",
                    "address": "Y",
                    "#text": "ES:[DI]"
                  },
                  {
                    "type": "wo",
                    "address": "X",
                    "#text": "DS:[SI]"
                  }
                ]
              },
              {
                "mnem": "CMPSW",
                "src": [
                  {
                    "type": "wo",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "ES:[DI]"
                  },
                  {
                    "type": "wo",
                    "address": "X",
                    "displayed": "no",
                    "#text": "DS:[SI]"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": [
              "arith",
              "string"
            ],
            "grp3": "binary",
            "test_f": "d",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare String Operands"
            }
          },
          {
            "op_size": "1",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "CMPS",
                "src": [
                  {
                    "type": "v",
                    "address": "Y",
                    "#text": "(ES:)[rDI]"
                  },
                  {
                    "type": "v",
                    "address": "X",
                    "#text": "(DS:)[rSI]"
                  }
                ]
              },
              {
                "mnem": "CMPSD",
                "src": [
                  {
                    "type": "do",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "(ES:)[rDI]"
                  },
                  {
                    "type": "do",
                    "address": "X",
                    "displayed": "no",
                    "#text": "(DS:)[rSI]"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": [
              "arith",
              "string"
            ],
            "grp3": "binary",
            "test_f": "d",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare String Operands"
            }
          },
          {
            "op_size": "1",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "CMPS",
                "src": [
                  {
                    "type": "vqp",
                    "address": "Y",
                    "#text": "[rDI]"
                  },
                  {
                    "type": "vqp",
                    "address": "X",
                    "#text": "[rSI]"
                  }
                ]
              },
              {
                "mnem": "CMPSW",
                "src": [
                  {
                    "type": "wo",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "[rDI]"
                  },
                  {
                    "type": "wo",
                    "address": "X",
                    "displayed": "no",
                    "#text": "[rSI]"
                  }
                ]
              },
              {
                "mnem": "CMPSD",
                "src": [
                  {
                    "type": "do",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "[rDI]"
                  },
                  {
                    "type": "do",
                    "address": "X",
                    "displayed": "no",
                    "#text": "[rSI]"
                  }
                ]
              },
              {
                "mnem": "CMPSQ",
                "src": [
                  {
                    "type": "qp",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "[rDI]"
                  },
                  {
                    "type": "qp",
                    "address": "X",
                    "displayed": "no",
                    "#text": "[rSI]"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": [
              "arith",
              "string"
            ],
            "grp3": "binary",
            "test_f": "d",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare String Operands"
            }
          }
        ]
      },
      {
        "value": "A8",
        "entry": {
          "op_size": "0",
          "attr": "acc",
          "syntax": {
            "mnem": "TEST",
            "src": [
              {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "#text": "AL"
              },
              {
                "a": "I",
                "t": "b"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Compare"
          }
        }
      },
      {
        "value": "A9",
        "entry": {
          "op_size": "1",
          "attr": "acc",
          "syntax": {
            "mnem": "TEST",
            "src": [
              {
                "nr": "0",
                "group": "gen",
                "type": "vqp",
                "#text": "rAX"
              },
              {
                "a": "I",
                "t": "vds"
              }
            ]
          },
          "grp1": "gen",
          "grp2": "logical",
          "modif_f": "oszapc",
          "def_f": "oszpc",
          "undef_f": "a",
          "f_vals": "oc",
          "note": {
            "brief": "Logical Compare"
          }
        }
      },
      {
        "value": "AA",
        "entry": {
          "op_size": "0",
          "syntax": [
            {
              "mnem": "STOS",
              "dst": {
                "type": "b",
                "address": "Y",
                "depend": "no",
                "#text": "(ES:)[rDI]"
              },
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              }
            },
            {
              "mnem": "STOSB",
              "dst": {
                "type": "b",
                "address": "Y",
                "displayed": "no",
                "depend": "no",
                "#text": "(ES:)[rDI]"
              },
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              }
            }
          ],
          "grp1": "gen",
          "grp2": [
            "datamov",
            "string"
          ],
          "test_f": "d",
          "note": {
            "brief": "Store String"
          }
        }
      },
      {
        "value": "AB",
        "entry": [
          {
            "op_size": "1",
            "syntax": [
              {
                "mnem": "STOS",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "depend": "no",
                  "#text": "ES:[DI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                }
              },
              {
                "mnem": "STOSW",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "ES:[DI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Store String"
            }
          },
          {
            "op_size": "1",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "STOS",
                "dst": {
                  "type": "v",
                  "address": "Y",
                  "depend": "no",
                  "#text": "(ES:)[rDI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "v",
                  "displayed": "no",
                  "#text": "eAX"
                }
              },
              {
                "mnem": "STOSD",
                "dst": {
                  "type": "do",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "(ES:)[rDI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EAX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Store String"
            }
          },
          {
            "op_size": "1",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "STOS",
                "dst": {
                  "type": "vqp",
                  "address": "Y",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rAX"
                }
              },
              {
                "mnem": "STOSW",
                "dst": {
                  "type": "wo",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "displayed": "no",
                  "#text": "AX"
                }
              },
              {
                "mnem": "STOSD",
                "dst": {
                  "type": "do",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "displayed": "no",
                  "#text": "EAX"
                }
              },
              {
                "mnem": "STOSQ",
                "dst": {
                  "type": "qp",
                  "address": "Y",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "[rDI]"
                },
                "src": {
                  "nr": "0",
                  "group": "gen",
                  "type": "qp",
                  "displayed": "no",
                  "#text": "RAX"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Store String"
            }
          }
        ]
      },
      {
        "value": "AC",
        "entry": {
          "op_size": "0",
          "syntax": [
            {
              "mnem": "LODS",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "depend": "no",
                "displayed": "no",
                "#text": "AL"
              },
              "src": {
                "type": "b",
                "address": "X",
                "#text": "(DS:)[rSI]"
              }
            },
            {
              "mnem": "LODSB",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "depend": "no",
                "displayed": "no",
                "#text": "AL"
              },
              "src": {
                "type": "b",
                "address": "X",
                "displayed": "no",
                "#text": "(DS:)[rSI]"
              }
            }
          ],
          "grp1": "gen",
          "grp2": [
            "datamov",
            "string"
          ],
          "test_f": "d",
          "note": {
            "brief": "Load String"
          }
        }
      },
      {
        "value": "AD",
        "entry": [
          {
            "op_size": "1",
            "syntax": [
              {
                "mnem": "LODS",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "AX"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "#text": "DS:[SI]"
                }
              },
              {
                "mnem": "LODSW",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "AX"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "displayed": "no",
                  "#text": "DS:[SI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Load String"
            }
          },
          {
            "op_size": "1",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "LODS",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "v",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "eAX"
                },
                "src": {
                  "type": "v",
                  "address": "X",
                  "#text": "(DS:)[rSI]"
                }
              },
              {
                "mnem": "LODSD",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EAX"
                },
                "src": {
                  "type": "do",
                  "address": "X",
                  "displayed": "no",
                  "#text": "(DS:)[rSI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Load String"
            }
          },
          {
            "op_size": "1",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "LODS",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "rAX"
                },
                "src": {
                  "type": "vqp",
                  "address": "X",
                  "#text": "[rSI]"
                }
              },
              {
                "mnem": "LODSW",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "wo",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "AX"
                },
                "src": {
                  "type": "wo",
                  "address": "X",
                  "displayed": "no",
                  "#text": "[rSI]"
                }
              },
              {
                "mnem": "LODSD",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "do",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EAX"
                },
                "src": {
                  "type": "do",
                  "address": "X",
                  "displayed": "no",
                  "#text": "[rSI]"
                }
              },
              {
                "mnem": "LODSQ",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "qp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "RAX"
                },
                "src": {
                  "type": "qp",
                  "address": "X",
                  "displayed": "no",
                  "#text": "[rSI]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "datamov",
              "string"
            ],
            "test_f": "d",
            "note": {
              "brief": "Load String"
            }
          }
        ]
      },
      {
        "value": "AE",
        "entry": {
          "op_size": "0",
          "syntax": [
            {
              "mnem": "SCAS",
              "src": [
                {
                  "type": "b",
                  "address": "Y",
                  "#text": "(ES:)[rDI]"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                }
              ]
            },
            {
              "mnem": "SCASB",
              "src": [
                {
                  "type": "b",
                  "address": "Y",
                  "displayed": "no",
                  "#text": "(ES:)[rDI]"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                }
              ]
            }
          ],
          "grp1": "gen",
          "grp2": [
            "arith",
            "string"
          ],
          "grp3": "binary",
          "test_f": "d",
          "modif_f": "oszapc",
          "def_f": "oszapc",
          "note": {
            "brief": "Scan String"
          }
        }
      },
      {
        "value": "AF",
        "entry": [
          {
            "op_size": "1",
            "syntax": [
              {
                "mnem": "SCAS",
                "src": [
                  {
                    "type": "wo",
                    "address": "Y",
                    "#text": "ES:[DI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "wo",
                    "displayed": "no",
                    "#text": "AX"
                  }
                ]
              },
              {
                "mnem": "SCASW",
                "src": [
                  {
                    "type": "wo",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "ES:[DI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "wo",
                    "displayed": "no",
                    "#text": "AX"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": [
              "arith",
              "string"
            ],
            "grp3": "binary",
            "test_f": "d",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Scan String"
            }
          },
          {
            "op_size": "1",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SCAS",
                "src": [
                  {
                    "type": "v",
                    "address": "Y",
                    "#text": "(ES:)[rDI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "v",
                    "displayed": "no",
                    "#text": "eAX"
                  }
                ]
              },
              {
                "mnem": "SCASD",
                "src": [
                  {
                    "type": "do",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "(ES:)[rDI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "do",
                    "displayed": "no",
                    "#text": "EAX"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": [
              "arith",
              "string"
            ],
            "grp3": "binary",
            "test_f": "d",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Scan String"
            }
          },
          {
            "op_size": "1",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "SCAS",
                "src": [
                  {
                    "type": "vqp",
                    "address": "Y",
                    "#text": "[rDI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "vqp",
                    "displayed": "no",
                    "#text": "rAX"
                  }
                ]
              },
              {
                "mnem": "SCASW",
                "src": [
                  {
                    "type": "wo",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "[rDI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "wo",
                    "displayed": "no",
                    "#text": "AX"
                  }
                ]
              },
              {
                "mnem": "SCASD",
                "src": [
                  {
                    "type": "do",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "[rDI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "do",
                    "displayed": "no",
                    "#text": "EAX"
                  }
                ]
              },
              {
                "mnem": "SCASQ",
                "src": [
                  {
                    "type": "qp",
                    "address": "Y",
                    "displayed": "no",
                    "#text": "[rDI]"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "qp",
                    "displayed": "no",
                    "#text": "RAX"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": [
              "arith",
              "string"
            ],
            "grp3": "binary",
            "test_f": "d",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Scan String"
            }
          }
        ]
      },
      {
        "value": "B0",
        "entry": {
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "Z",
              "t": "b"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "B8",
        "entry": {
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "Z",
              "t": "vqp"
            },
            "src": {
              "a": "I",
              "t": "vqp"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "C0",
        "proc_start": "01",
        "entry": [
          {
            "op_size": "0",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ROL",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "ROR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "RCL",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "RCR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "4",
            "syntax": [
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SHR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "alias": "C0_4",
            "doc": "u",
            "doc_ref": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4",
            "opcd_ext": "6",
            "syntax": [
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "SAR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oa",
            "note": {
              "brief": "Shift"
            }
          }
        ]
      },
      {
        "value": "C1",
        "proc_start": "01",
        "entry": [
          {
            "op_size": "1",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ROL",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "ROR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "RCL",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "RCR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "4",
            "syntax": [
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SHR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "alias": "C1_4",
            "doc": "u",
            "doc_ref": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4",
            "opcd_ext": "6",
            "syntax": [
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "SAR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oa",
            "note": {
              "brief": "Shift"
            }
          }
        ]
      },
      {
        "value": "C2",
        "entry": {
          "syntax": {
            "mnem": "RETN",
            "src": [
              {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              {
                "a": "I",
                "t": "w"
              }
            ]
          },
          "grp1": "gen",
          "grp2": [
            "branch",
            "stack"
          ],
          "note": {
            "brief": "Return from procedure"
          }
        }
      },
      {
        "value": "C3",
        "entry": {
          "syntax": {
            "mnem": "RETN",
            "src": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            }
          },
          "grp1": "gen",
          "grp2": [
            "branch",
            "stack"
          ],
          "note": {
            "brief": "Return from procedure"
          }
        }
      },
      {
        "value": "C4",
        "entry": [
          {
            "r": "yes",
            "syntax": {
              "mnem": "LES",
              "dst": [
                {
                  "nr": "0",
                  "group": "seg",
                  "type": "w",
                  "displayed": "no",
                  "#text": "ES"
                },
                {
                  "depend": "no",
                  "a": "G",
                  "t": "v"
                }
              ],
              "src": {
                "a": "M",
                "t": "p"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "segreg"
            ],
            "note": {
              "brief": "Load Far Pointer"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "C5",
        "entry": [
          {
            "r": "yes",
            "syntax": {
              "mnem": "LDS",
              "dst": [
                {
                  "nr": "3",
                  "group": "seg",
                  "type": "w",
                  "displayed": "no",
                  "#text": "DS"
                },
                {
                  "depend": "no",
                  "a": "G",
                  "t": "v"
                }
              ],
              "src": {
                "a": "M",
                "t": "p"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "segreg"
            ],
            "note": {
              "brief": "Load Far Pointer"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "C6",
        "entry": {
          "op_size": "0",
          "opcd_ext": "0",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "E",
              "t": "b"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "C7",
        "entry": {
          "op_size": "1",
          "opcd_ext": "0",
          "syntax": {
            "mnem": "MOV",
            "dst": {
              "depend": "no",
              "a": "E",
              "t": "vqp"
            },
            "src": {
              "a": "I",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Move"
          }
        }
      },
      {
        "value": "C8",
        "entry": [
          {
            "proc_start": "01",
            "syntax": {
              "mnem": "ENTER",
              "dst": [
                {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                },
                {
                  "nr": "5",
                  "group": "gen",
                  "type": "v",
                  "displayed": "no",
                  "#text": "eBP"
                }
              ],
              "src": [
                {
                  "a": "I",
                  "t": "w"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Make Stack Frame for Procedure Parameters"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "ENTER",
              "dst": [
                {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                },
                {
                  "nr": "5",
                  "group": "gen",
                  "type": "vq",
                  "displayed": "no",
                  "#text": "rBP"
                }
              ],
              "src": [
                {
                  "a": "I",
                  "t": "w"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Make Stack Frame for Procedure Parameters"
            }
          }
        ]
      },
      {
        "value": "C9",
        "entry": [
          {
            "proc_start": "01",
            "syntax": {
              "mnem": "LEAVE",
              "dst": {
                "nr": "5",
                "group": "gen",
                "type": "v",
                "displayed": "no",
                "#text": "eBP"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "High Level Procedure Exit"
            }
          },
          {
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "LEAVE",
              "dst": {
                "nr": "5",
                "group": "gen",
                "type": "vq",
                "displayed": "no",
                "#text": "rBP"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "High Level Procedure Exit"
            }
          }
        ]
      },
      {
        "value": "CA",
        "entry": {
          "ring": "f",
          "syntax": {
            "mnem": "RETF",
            "src": [
              {
                "a": "I",
                "t": "w"
              },
              {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            ]
          },
          "grp1": "gen",
          "grp2": [
            "branch",
            "stack"
          ],
          "note": {
            "brief": "Return from procedure"
          }
        }
      },
      {
        "value": "CB",
        "entry": {
          "ring": "f",
          "syntax": {
            "mnem": "RETF",
            "src": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            }
          },
          "grp1": "gen",
          "grp2": [
            "branch",
            "stack"
          ],
          "note": {
            "brief": "Return from procedure"
          }
        }
      },
      {
        "value": "CC",
        "entry": {
          "alias": "CD",
          "ring": "f",
          "syntax": {
            "mnem": "INT",
            "dst": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            },
            "src": [
              {
                "address": "I",
                "#text": "3"
              },
              {
                "type": "v",
                "address": "F",
                "displayed": "no",
                "#text": "eFlags"
              }
            ]
          },
          "grp1": "gen",
          "grp2": [
            "break",
            "stack"
          ],
          "modif_f": "i",
          "def_f": "i",
          "f_vals": "i",
          "note": {
            "brief": "Call to Interrupt Procedure"
          }
        }
      },
      {
        "value": "CD",
        "entry": {
          "ring": "f",
          "syntax": {
            "mnem": "INT",
            "dst": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            },
            "src": [
              {
                "a": "I",
                "t": "b"
              },
              {
                "type": "v",
                "address": "F",
                "displayed": "no",
                "#text": "eFlags"
              }
            ]
          },
          "grp1": "gen",
          "grp2": [
            "break",
            "stack"
          ],
          "modif_f": "i",
          "def_f": "i",
          "f_vals": "i",
          "note": {
            "brief": "Call to Interrupt Procedure"
          }
        }
      },
      {
        "value": "CE",
        "entry": {
          "ring": "f",
          "syntax": {
            "mnem": "INTO",
            "dst": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            },
            "src": {
              "type": "v",
              "address": "F",
              "displayed": "no",
              "#text": "eFlags"
            }
          },
          "grp1": "gen",
          "grp2": [
            "break",
            "stack"
          ],
          "test_f": "o",
          "modif_f": {
            "cond": "yes",
            "#text": "i"
          },
          "def_f": {
            "cond": "yes",
            "#text": "i"
          },
          "f_vals": "i",
          "note": {
            "brief": "Call to Interrupt Procedure"
          }
        }
      },
      {
        "value": "CF",
        "entry": [
          {
            "ring": "f",
            "syntax": {
              "mnem": "IRET",
              "dst": {
                "type": "wo",
                "address": "F",
                "displayed": "no",
                "#text": "Flags"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "break",
              "stack"
            ],
            "note": {
              "brief": "Interrupt Return"
            }
          },
          {
            "ring": "f",
            "proc_start": "03",
            "syntax": {
              "mnem": "IRETD",
              "dst": {
                "type": "do",
                "address": "F",
                "displayed": "no",
                "#text": "EFlags"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "break",
              "stack"
            ],
            "note": {
              "brief": "Interrupt Return"
            }
          },
          {
            "ring": "f",
            "mode": "e",
            "syntax": [
              {
                "mnem": "IRET",
                "dst": {
                  "type": "wo",
                  "address": "F",
                  "displayed": "no",
                  "#text": "Flags"
                },
                "src": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                }
              },
              {
                "mnem": "IRETD",
                "dst": {
                  "type": "do",
                  "address": "F",
                  "displayed": "no",
                  "#text": "EFlags"
                },
                "src": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                }
              },
              {
                "mnem": "IRETQ",
                "dst": {
                  "type": "qp",
                  "address": "F",
                  "displayed": "no",
                  "#text": "RFlags"
                },
                "src": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "break",
              "stack"
            ],
            "note": {
              "brief": "Interrupt Return"
            }
          }
        ]
      },
      {
        "value": "D0",
        "entry": [
          {
            "op_size": "0",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ROL",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "ROR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "RCL",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "RCR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "4",
            "syntax": [
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              },
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SHR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "alias": "D0_4",
            "doc": "u",
            "doc_ref": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4",
            "opcd_ext": "6",
            "syntax": [
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              },
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "SAR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          }
        ]
      },
      {
        "value": "D1",
        "entry": [
          {
            "op_size": "1",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ROL",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "ROR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "RCL",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "RCR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "4",
            "syntax": [
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              },
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SHR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "alias": "D1_4",
            "doc": "u",
            "doc_ref": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4",
            "opcd_ext": "6",
            "syntax": [
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              },
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "address": "I",
                  "#text": "1"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "SAR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "address": "I",
                "#text": "1"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          }
        ]
      },
      {
        "value": "D2",
        "entry": [
          {
            "op_size": "0",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ROL",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "ROR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "RCL",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "RCR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "4",
            "syntax": [
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              },
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SHR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "alias": "D2_4",
            "doc": "u",
            "doc_ref": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4",
            "opcd_ext": "6",
            "syntax": [
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              },
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "b"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "SAR",
              "dst": {
                "a": "E",
                "t": "b"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oa",
            "note": {
              "brief": "Shift"
            }
          }
        ]
      },
      {
        "value": "D3",
        "entry": [
          {
            "op_size": "1",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "ROL",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "ROR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "RCL",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "RCR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "test_f": "c",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "undef_f": "o",
            "note": {
              "brief": "Rotate"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "4",
            "syntax": [
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              },
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "SHR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "alias": "D3_4",
            "doc": "u",
            "doc_ref": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4",
            "opcd_ext": "6",
            "syntax": [
              {
                "mnem": "SAL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              },
              {
                "mnem": "SHL",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Shift"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "SAR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "nr": "1",
                "group": "gen",
                "type": "b",
                "#text": "CL"
              }
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "note": {
              "brief": "Shift"
            }
          }
        ]
      },
      {
        "value": "D4",
        "entry": [
          {
            "sec_opcd": "0A",
            "syntax": {
              "mnem": "AAM",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "AH"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "modif_f": "oszapc",
            "def_f": "szp",
            "undef_f": "oac",
            "note": {
              "brief": "ASCII Adjust AX After Multiply"
            }
          },
          {
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "AMX"
              },
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "AH"
                }
              ],
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "modif_f": "oszapc",
            "def_f": "szp",
            "undef_f": "oac",
            "note": {
              "brief": "Adjust AX After Multiply"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "D5",
        "entry": [
          {
            "sec_opcd": "0A",
            "syntax": {
              "mnem": "AAD",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AH"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "modif_f": "oszapc",
            "def_f": "szp",
            "undef_f": "oac",
            "note": {
              "brief": "ASCII Adjust AX Before Division"
            }
          },
          {
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "ADX"
              },
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AH"
                }
              ],
              "src": {
                "a": "I",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "decimal",
            "modif_f": "oszapc",
            "def_f": "szp",
            "undef_f": "oac",
            "note": {
              "brief": "Adjust AX Before Division"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "D6",
        "entry": [
          {
            "attr": "undef",
            "is_undoc": "yes",
            "doc_ref": "gen_note_undefined_D6_F1",
            "particular": "yes",
            "proc_start": "02",
            "syntax": [],
            "note": {
              "brief": "Undefined and Reserved; Does not Generate #UD"
            }
          },
          {
            "doc": "u",
            "is_doc": "yes",
            "doc_ref": "gen_note_u_SALC_D6",
            "proc_start": "02",
            "syntax": [
              {
                "mnem": "SALC",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "AL"
                }
              },
              {
                "mnem": "SETALC",
                "dst": {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "AL"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "c",
            "note": {
              "brief": "Set AL If Carry"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "D7",
        "entry": {
          "syntax": [
            {
              "mnem": "XLAT",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              },
              "src": {
                "type": "b",
                "address": "BB",
                "#text": "(DS:)[rBX+AL]"
              }
            },
            {
              "mnem": "XLATB",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "b",
                "displayed": "no",
                "#text": "AL"
              },
              "src": {
                "type": "b",
                "address": "BB",
                "displayed": "no",
                "#text": "(DS:)[rBX+AL]"
              }
            }
          ],
          "grp1": "gen",
          "grp2": "datamov",
          "note": {
            "brief": "Table Look-up Translation"
          }
        }
      },
      {
        "value": "D8",
        "entry": [
          {
            "mem_format": "00",
            "opcd_ext": "0",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "FADD",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                "src": {
                  "a": "M",
                  "t": "sr"
                }
              },
              {
                "mod": "nomem",
                "mnem": "FADD",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                "src": {
                  "a": "EST"
                }
              }
            ],
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "1",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "FMUL",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                "src": {
                  "a": "M",
                  "t": "sr"
                }
              },
              {
                "mod": "nomem",
                "mnem": "FMUL",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                "src": {
                  "a": "EST"
                }
              }
            ],
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FCOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "ES",
                  "t": "sr"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real"
            }
          },
          {
            "opcd_ext": "2",
            "sec_opcd": "D1",
            "syntax": {
              "mnem": "FCOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "address": "EST",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real"
            }
          },
          {
            "mem_format": "00",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FCOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "ES",
                  "t": "sr"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "3",
            "sec_opcd": "D9",
            "syntax": {
              "mnem": "FCOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "address": "EST",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "4",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "FSUB",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                "src": {
                  "a": "M",
                  "t": "sr"
                }
              },
              {
                "mod": "nomem",
                "mnem": "FSUB",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                "src": {
                  "a": "EST"
                }
              }
            ],
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "5",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "FSUBR",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                "src": {
                  "a": "M",
                  "t": "sr"
                }
              },
              {
                "mod": "nomem",
                "mnem": "FSUBR",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                "src": {
                  "a": "EST"
                }
              }
            ],
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "6",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "FDIV",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                "src": {
                  "a": "M",
                  "t": "sr"
                }
              },
              {
                "mod": "nomem",
                "mnem": "FDIV",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                "src": {
                  "a": "EST"
                }
              }
            ],
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "7",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "FDIVR",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                "src": {
                  "a": "M",
                  "t": "sr"
                }
              },
              {
                "mod": "nomem",
                "mnem": "FDIVR",
                "dst": {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                "src": {
                  "a": "EST"
                }
              }
            ],
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide"
            }
          }
        ]
      },
      {
        "value": "D9",
        "entry": [
          {
            "mem_format": "00",
            "fpush": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FLD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "ES",
                "t": "sr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Floating Point Value"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "FXCH",
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Exchange Register Contents"
            }
          },
          {
            "opcd_ext": "1",
            "sec_opcd": "C9",
            "syntax": {
              "mnem": "FXCH",
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "address": "EST",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Exchange Register Contents"
            }
          },
          {
            "mem_format": "00",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FST",
              "dst": {
                "a": "M",
                "t": "sr"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value"
            }
          },
          {
            "opcd_ext": "2",
            "sec_opcd": "D0",
            "syntax": {
              "mnem": "FNOP"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "No Operation"
            }
          },
          {
            "mem_format": "00",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FSTP",
              "dst": {
                "a": "M",
                "t": "sr"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mod": "nomem",
            "part_alias": "D9_3",
            "doc_part_alias_ref": "gen_note_FSTP1_D9_3_not_true_alias",
            "doc_ref": "gen_note_FSTP1_D9_3_FSTP8_DF_2_FSTP9_DF_3",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "3",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FSTP1"
              },
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mod": "nomem",
            "part_alias": "D9_3",
            "doc_part_alias_ref": "gen_note_FSTP1_D9_3_not_true_alias",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "3",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FSTP1"
              },
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FLDENV",
              "src": {
                "a": "M",
                "t": "e"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Load x87 FPU Environment"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E0",
            "syntax": {
              "mnem": "FCHS",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Change Sign"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E1",
            "syntax": {
              "mnem": "FABS",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Absolute Value"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E4",
            "syntax": {
              "mnem": "FTST",
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Test"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E5",
            "syntax": {
              "mnem": "FXAM",
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Examine"
            }
          },
          {
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FLDCW",
              "src": {
                "a": "M",
                "t": "w"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Load x87 FPU Control Word"
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "E8",
            "syntax": {
              "mnem": "FLD1",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Constant +1.0"
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "E9",
            "syntax": {
              "mnem": "FLDL2T",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Load Constant log",
                  "10"
                ],
                "sub": "2"
              }
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "EA",
            "syntax": {
              "mnem": "FLDL2E",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Load Constant log",
                  "e"
                ],
                "sub": "2"
              }
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "EB",
            "syntax": {
              "mnem": "FLDPI",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Constant π"
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "EC",
            "syntax": {
              "mnem": "FLDLG2",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Load Constant log",
                  "2"
                ],
                "sub": "10"
              }
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "ED",
            "syntax": {
              "mnem": "FLDLN2",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Load Constant log",
                  "2"
                ],
                "sub": "e"
              }
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "5",
            "sec_opcd": "EE",
            "syntax": {
              "mnem": "FLDZ",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "ldconst",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Constant +0.0"
            }
          },
          {
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FNSTENV",
              "dst": {
                "depend": "no",
                "a": "M",
                "t": "e"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Environment"
            }
          },
          {
            "opcd_ext": "6",
            "pref": "9B",
            "syntax": {
              "mnem": "FSTENV",
              "dst": {
                "depend": "no",
                "a": "M",
                "t": "e"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Environment"
            }
          },
          {
            "opcd_ext": "6",
            "sec_opcd": "F0",
            "syntax": {
              "mnem": "F2XM1",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Compute 2",
                  "-1"
                ],
                "sup": "x"
              }
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "6",
            "sec_opcd": "F1",
            "syntax": {
              "mnem": "FYL2X",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Compute y × log",
                  "x and Pop"
                ],
                "sub": "2"
              }
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "6",
            "sec_opcd": "F2",
            "syntax": {
              "mnem": "FPTAN",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "12",
            "undef_f_fpu": "03",
            "note": {
              "brief": "Partial Tangent"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "6",
            "sec_opcd": "F3",
            "syntax": {
              "mnem": "FPATAN",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Partial Arctangent and Pop"
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "6",
            "sec_opcd": "F4",
            "syntax": {
              "mnem": "FXTRACT",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Extract Exponent and Significand"
            }
          },
          {
            "opcd_ext": "6",
            "sec_opcd": "F5",
            "syntax": {
              "mnem": "FPREM1",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "nr": "1",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST1"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "IEEE Partial Remainder"
            }
          },
          {
            "opcd_ext": "6",
            "sec_opcd": "F6",
            "syntax": {
              "mnem": "FDECSTP"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "f_vals_fpu": "b",
            "note": {
              "brief": "Decrement Stack-Top Pointer"
            }
          },
          {
            "opcd_ext": "6",
            "sec_opcd": "F7",
            "syntax": {
              "mnem": "FINCSTP"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "f_vals_fpu": "b",
            "note": {
              "brief": "Increment Stack-Top Pointer"
            }
          },
          {
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FNSTCW",
              "dst": {
                "depend": "no",
                "a": "M",
                "t": "w"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Control Word"
            }
          },
          {
            "opcd_ext": "7",
            "pref": "9B",
            "syntax": {
              "mnem": "FSTCW",
              "dst": {
                "depend": "no",
                "a": "M",
                "t": "w"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Control Word"
            }
          },
          {
            "opcd_ext": "7",
            "sec_opcd": "F8",
            "syntax": {
              "mnem": "FPREM",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "nr": "1",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST1"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Partial Remainder (for compatibility with i8087 and i287)"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "7",
            "sec_opcd": "F9",
            "syntax": {
              "mnem": "FYL2XP1",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": {
                "#text": [
                  "Compute y × log",
                  "(x+1) and Pop"
                ],
                "sub": "2"
              }
            }
          },
          {
            "opcd_ext": "7",
            "sec_opcd": "FA",
            "syntax": {
              "mnem": "FSQRT",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Square Root"
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "7",
            "sec_opcd": "FB",
            "syntax": {
              "mnem": "FSINCOS",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "12",
            "undef_f_fpu": "03",
            "note": {
              "brief": "Sine and Cosine"
            }
          },
          {
            "opcd_ext": "7",
            "sec_opcd": "FC",
            "syntax": {
              "mnem": "FRNDINT",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Round to Integer"
            }
          },
          {
            "opcd_ext": "7",
            "sec_opcd": "FD",
            "syntax": {
              "mnem": "FSCALE",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "nr": "1",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST1"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Scale"
            }
          },
          {
            "opcd_ext": "7",
            "sec_opcd": "FE",
            "syntax": {
              "mnem": "FSIN",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "12",
            "undef_f_fpu": "03",
            "note": {
              "brief": "Sine"
            }
          },
          {
            "opcd_ext": "7",
            "sec_opcd": "FF",
            "syntax": {
              "mnem": "FCOS",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "trans",
            "modif_f_fpu": "0123",
            "def_f_fpu": "12",
            "undef_f_fpu": "03",
            "note": {
              "brief": "Cosine"
            }
          }
        ]
      },
      {
        "value": "DA",
        "entry": [
          {
            "mem_format": "01",
            "mod": "mem",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FIADD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "0",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVB",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "c",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - below (CF=1)"
            }
          },
          {
            "mem_format": "01",
            "mod": "mem",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "FIMUL",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "1",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVE",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "z",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - equal (ZF=1)"
            }
          },
          {
            "mem_format": "01",
            "mod": "mem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FICOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "M",
                  "t": "di"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Integer"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "2",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVBE",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "z",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - below or equal (CF=1 or ZF=1)"
            }
          },
          {
            "mem_format": "01",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FICOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "M",
                  "t": "di"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Integer and Pop"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "3",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVU",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "p",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - unordered (PF=1)"
            }
          },
          {
            "mem_format": "01",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FISUB",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "mem_format": "01",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FISUBR",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract"
            }
          },
          {
            "fpop": "twice",
            "opcd_ext": "5",
            "sec_opcd": "E9",
            "proc_start": "03",
            "syntax": {
              "mnem": "FUCOMPP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Unordered Compare Floating Point Values and Pop Twice"
            }
          },
          {
            "mem_format": "01",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FIDIV",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide"
            }
          },
          {
            "mem_format": "01",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FIDIVR",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide"
            }
          }
        ]
      },
      {
        "value": "DB",
        "entry": [
          {
            "mem_format": "01",
            "mod": "mem",
            "fpush": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FILD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "di"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Integer"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "0",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVNB",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "c",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - not below (CF=0)"
            }
          },
          {
            "mem_format": "01",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "1",
            "proc_start": {
              "lat_step": "yes",
              "#text": "10"
            },
            "syntax": {
              "mnem": "FISTTP",
              "dst": {
                "a": "M",
                "t": "di"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "instr_ext": "sse3",
            "grp1": "x87fpu",
            "grp2": "conver",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "f_vals_fpu": "b",
            "note": {
              "brief": "Store Integer with Truncation and Pop"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "1",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVNE",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "z",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - not equal (ZF=0)"
            }
          },
          {
            "mem_format": "01",
            "mod": "mem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FIST",
              "dst": {
                "a": "M",
                "t": "di"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Integer"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "2",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVNBE",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "z",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - below or equal (CF=0 and ZF=0)"
            }
          },
          {
            "mem_format": "01",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FISTP",
              "dst": {
                "a": "M",
                "t": "di"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Integer and Pop"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "3",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCMOVNU",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "test_f": "p",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "FP Conditional Move - not unordered (PF=0)"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E0",
            "proc_start": {
              "post": "no",
              "#text": "00"
            },
            "proc_end": "00",
            "syntax": {
              "mnem": "FNENI"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Enable NPX Interrupt"
            }
          },
          {
            "opcd_ext": "4",
            "pref": "9B",
            "sec_opcd": "E0",
            "proc_start": {
              "post": "no",
              "#text": "00"
            },
            "proc_end": "00",
            "syntax": {
              "mnem": "FENI"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Enable NPX Interrupt"
            }
          },
          {
            "attr": "nop",
            "doc_ref": "gen_note_FNENI_DBE0_FNDISI_DBE1",
            "particular": "yes",
            "opcd_ext": "4",
            "sec_opcd": "E0",
            "proc_start": "01",
            "syntax": {
              "mnem": "FNENI"
            },
            "grp1": "obsol",
            "grp2": "control",
            "note": {
              "brief": "Treated as Integer NOP"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E1",
            "proc_start": {
              "post": "no",
              "#text": "00"
            },
            "proc_end": "00",
            "syntax": {
              "mnem": "FNDISI"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Disable NPX Interrupt"
            }
          },
          {
            "opcd_ext": "4",
            "pref": "9B",
            "sec_opcd": "E1",
            "proc_start": {
              "post": "no",
              "#text": "00"
            },
            "proc_end": "00",
            "syntax": {
              "mnem": "FDISI"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Disable NPX Interrupt"
            }
          },
          {
            "attr": "nop",
            "doc_ref": "gen_note_FNENI_DBE0_FNDISI_DBE1",
            "particular": "yes",
            "opcd_ext": "4",
            "sec_opcd": "E1",
            "proc_start": "01",
            "syntax": {
              "mnem": "FNDISI"
            },
            "grp1": "obsol",
            "grp2": "control",
            "note": {
              "brief": "Treated as Integer NOP"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E2",
            "syntax": {
              "mnem": "FNCLEX"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Clear Exceptions"
            }
          },
          {
            "opcd_ext": "4",
            "pref": "9B",
            "sec_opcd": "E2",
            "syntax": {
              "mnem": "FCLEX"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Clear Exceptions"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E3",
            "syntax": {
              "mnem": "FNINIT"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "f_vals_fpu": "abcd",
            "note": {
              "brief": "Initialize Floating-Point Unit"
            }
          },
          {
            "opcd_ext": "4",
            "pref": "9B",
            "sec_opcd": "E3",
            "syntax": {
              "mnem": "FINIT"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "f_vals_fpu": "abcd",
            "note": {
              "brief": "Initialize Floating-Point Unit"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E4",
            "proc_start": {
              "post": "no",
              "#text": "02"
            },
            "proc_end": "02",
            "syntax": {
              "mnem": "FNSETPM"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Set Protected Mode"
            }
          },
          {
            "opcd_ext": "4",
            "pref": "9B",
            "sec_opcd": "E4",
            "proc_start": {
              "post": "no",
              "#text": "02"
            },
            "proc_end": "02",
            "syntax": {
              "mnem": "FSETPM"
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Set Protected Mode"
            }
          },
          {
            "attr": "nop",
            "doc_ref": "gen_note_FNSETPM_DBE4",
            "particular": "yes",
            "opcd_ext": "4",
            "sec_opcd": "E4",
            "proc_start": "03",
            "syntax": {
              "mnem": "FNSETPM"
            },
            "grp1": "obsol",
            "grp2": "control",
            "note": {
              "brief": "Treated as Integer NOP"
            }
          },
          {
            "mod": "mem",
            "fpush": "yes",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FLD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "er"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Floating Point Value"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "5",
            "proc_start": "07",
            "syntax": {
              "mnem": "FUCOMI",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f": "ozpc",
            "def_f": "ozpc",
            "f_vals": "o",
            "modif_f_fpu": "1",
            "def_f_fpu": "1",
            "note": {
              "brief": "Unordered Compare Floating Point Values and Set EFLAGS"
            }
          },
          {
            "opcd_ext": "6",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCOMI",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f": "ozpc",
            "def_f": "ozpc",
            "f_vals": "o",
            "modif_f_fpu": "1",
            "def_f_fpu": "1",
            "note": {
              "brief": "Compare Floating Point Values and Set EFLAGS"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FSTP",
              "dst": {
                "a": "M",
                "t": "er"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          }
        ]
      },
      {
        "value": "DC",
        "entry": [
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FADD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FADD",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "FMUL",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "FMUL",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FCOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "M",
                  "t": "dr"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real"
            }
          },
          {
            "mod": "nomem",
            "alias": "D8_2",
            "doc_ref": "gen_note_FCOM2_DC_2",
            "particular": "yes",
            "opcd_ext": "2",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FCOM2"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real"
            }
          },
          {
            "mod": "nomem",
            "alias": "D8_2",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "particular": "yes",
            "opcd_ext": "2",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FCOM2"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FCOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "M",
                  "t": "dr"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D8_3",
            "doc_ref": "gen_note_FCOMP3_DC_3_FCOMP5_DE_2",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "3",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FCOMP3"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D8_3",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "3",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FCOMP3"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FSUB",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FSUBR",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FSUBR",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FSUB",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FDIV",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FDIVR",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FDIVR",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FDIV",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide and Pop"
            }
          }
        ]
      },
      {
        "value": "DD",
        "entry": [
          {
            "mem_format": "10",
            "mod": "mem",
            "fpush": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FLD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "dr"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Floating Point Value"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FFREE",
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Free Floating-Point Register"
            }
          },
          {
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "1",
            "proc_start": {
              "lat_step": "yes",
              "#text": "10"
            },
            "syntax": {
              "mnem": "FISTTP",
              "dst": {
                "a": "M",
                "t": "qi"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "instr_ext": "sse3",
            "grp1": "x87fpu",
            "grp2": "conver",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "f_vals_fpu": "b",
            "note": {
              "brief": "Store Integer with Truncation and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_1",
            "doc_ref": "gen_note_FXCH4_DD_1_FXCH7_DF_1",
            "particular": "yes",
            "opcd_ext": "1",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FXCH4"
              },
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Exchange Register Contents"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_1",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "particular": "yes",
            "opcd_ext": "1",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FXCH4"
              },
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Exchange Register Contents"
            }
          },
          {
            "mem_format": "10",
            "mod": "mem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FST",
              "dst": {
                "a": "M",
                "t": "dr"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FST",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value"
            }
          },
          {
            "mem_format": "10",
            "fpop": "once",
            "mod": "mem",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FSTP",
              "dst": {
                "a": "M",
                "t": "dr"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FSTP",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mod": "mem",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FRSTOR",
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST1"
                },
                {
                  "nr": "2",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST2"
                },
                {
                  "nr": "3",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST3"
                },
                {
                  "nr": "4",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST4"
                },
                {
                  "nr": "5",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST5"
                },
                {
                  "nr": "6",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST6"
                },
                {
                  "nr": "7",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST7"
                }
              ],
              "src": {
                "a": "M",
                "t": "st"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Restore x87 FPU State"
            }
          },
          {
            "mod": "nomem",
            "opcd_ext": "4",
            "proc_start": "03",
            "syntax": {
              "mnem": "FUCOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Unordered Compare Floating Point Values"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E1",
            "proc_start": "03",
            "syntax": {
              "mnem": "FUCOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "address": "EST",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Unordered Compare Floating Point Values"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "5",
            "proc_start": "03",
            "syntax": {
              "mnem": "FUCOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Unordered Compare Floating Point Values and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "5",
            "sec_opcd": "E9",
            "proc_start": "03",
            "syntax": {
              "mnem": "FUCOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "address": "EST",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Unordered Compare Floating Point Values and Pop"
            }
          },
          {
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FNSAVE",
              "dst": {
                "a": "M",
                "t": "st"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST1"
                },
                {
                  "nr": "2",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST2"
                },
                {
                  "nr": "3",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST3"
                },
                {
                  "nr": "4",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST4"
                },
                {
                  "nr": "5",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST5"
                },
                {
                  "nr": "6",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST6"
                },
                {
                  "nr": "7",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST7"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "f_vals_fpu": "abcd",
            "note": {
              "brief": "Store x87 FPU State"
            }
          },
          {
            "opcd_ext": "6",
            "pref": "9B",
            "syntax": {
              "mnem": "FSAVE",
              "dst": {
                "a": "M",
                "t": "st"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST1"
                },
                {
                  "nr": "2",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST2"
                },
                {
                  "nr": "3",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST3"
                },
                {
                  "nr": "4",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST4"
                },
                {
                  "nr": "5",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST5"
                },
                {
                  "nr": "6",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST6"
                },
                {
                  "nr": "7",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST7"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "f_vals_fpu": "abcd",
            "note": {
              "brief": "Store x87 FPU State"
            }
          },
          {
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FNSTSW",
              "dst": {
                "a": "M",
                "t": "w"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Status Word"
            }
          },
          {
            "opcd_ext": "7",
            "pref": "9B",
            "syntax": {
              "mnem": "FSTSW",
              "dst": {
                "a": "M",
                "t": "w"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Status Word"
            }
          }
        ]
      },
      {
        "value": "DE",
        "entry": [
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FIADD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FADDP",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "0",
            "sec_opcd": "C1",
            "syntax": {
              "mnem": "FADDP",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "address": "EST",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Add and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "FIMUL",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "FMULP",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "1",
            "sec_opcd": "C9",
            "syntax": {
              "mnem": "FMULP",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "address": "EST",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Multiply and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FICOM",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "M",
                  "t": "wi"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Integer"
            }
          },
          {
            "mod": "nomem",
            "alias": "D8_3",
            "doc_ref": "gen_note_FCOMP3_DC_3_FCOMP5_DE_2",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "2",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FCOMP5"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D8_3",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "2",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FCOMP5"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop"
            }
          },
          {
            "mem_format": "11",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FICOMP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "M",
                  "t": "wi"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Integer and Pop"
            }
          },
          {
            "fpop": "twice",
            "opcd_ext": "3",
            "sec_opcd": "D9",
            "syntax": {
              "mnem": "FCOMPP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "nr": "1",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST1"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f_fpu": "0123",
            "def_f_fpu": "0123",
            "note": {
              "brief": "Compare Real and Pop Twice"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FISUB",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FSUBRP",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "4",
            "sec_opcd": "E1",
            "syntax": {
              "mnem": "FSUBRP",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "address": "EST",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FISUBR",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Subtract"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FSUBP",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "5",
            "sec_opcd": "E9",
            "syntax": {
              "mnem": "FSUBP",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "address": "EST",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Subtract and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FIDIV",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FDIVRP",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "6",
            "sec_opcd": "F1",
            "syntax": {
              "mnem": "FDIVRP",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "address": "EST",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FIDIVR",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Reverse Divide"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FDIVP",
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "7",
            "sec_opcd": "F9",
            "syntax": {
              "mnem": "FDIVP",
              "dst": {
                "nr": "1",
                "group": "x87fpu",
                "address": "EST",
                "displayed": "no",
                "#text": "ST1"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "arith",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Divide and Pop"
            }
          }
        ]
      },
      {
        "value": "DF",
        "entry": [
          {
            "mem_format": "11",
            "mod": "mem",
            "fpush": "yes",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "FILD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "wi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Integer"
            }
          },
          {
            "mod": "nomem",
            "doc_ref": "gen_note_FFREEP_DF_1",
            "fpop": "once",
            "opcd_ext": "0",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FFREEP"
              },
              "src": {
                "a": "EST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Free Floating-Point Register and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "1",
            "proc_start": {
              "lat_step": "yes",
              "#text": "10"
            },
            "syntax": {
              "mnem": "FISTTP",
              "dst": {
                "a": "M",
                "t": "wi"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "instr_ext": "sse3",
            "grp1": "x87fpu",
            "grp2": "conver",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "f_vals_fpu": "b",
            "note": {
              "brief": "Store Integer with Truncation and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_1",
            "doc_ref": "gen_note_FXCH4_DD_1_FXCH7_DF_1",
            "particular": "yes",
            "opcd_ext": "1",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FXCH7"
              },
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Exchange Register Contents"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_1",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "particular": "yes",
            "opcd_ext": "1",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FXCH7"
              },
              "dst": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "displayed": "no",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Exchange Register Contents"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "FIST",
              "dst": {
                "a": "M",
                "t": "wi"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Integer"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_3",
            "doc_ref": "gen_note_FSTP1_D9_3_FSTP8_DF_2_FSTP9_DF_3",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "2",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FSTP8"
              },
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_3",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "2",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FSTP8"
              },
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mem_format": "11",
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "FISTP",
              "dst": {
                "a": "M",
                "t": "wi"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Integer and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_3",
            "doc_ref": "gen_note_FSTP1_D9_3_FSTP8_DF_2_FSTP9_DF_3",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "3",
            "proc_end": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FSTP9"
              },
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "mod": "nomem",
            "alias": "D9_3",
            "doc": "u",
            "doc_ref": "gen_note_x87_fpu_undoc_aliases",
            "fpop": "once",
            "particular": "yes",
            "opcd_ext": "3",
            "proc_start": "03",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "FSTP9"
              },
              "dst": {
                "a": "EST"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Floating Point Value and Pop"
            }
          },
          {
            "fpush": "yes",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "FBLD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "bcd"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Binary Coded Decimal"
            }
          },
          {
            "opcd_ext": "4",
            "sec_opcd": "E0",
            "proc_start": "02",
            "syntax": {
              "mnem": "FNSTSW",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "w",
                "depend": "no",
                "#text": "AX"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Status Word"
            }
          },
          {
            "opcd_ext": "4",
            "pref": "9B",
            "sec_opcd": "E0",
            "proc_start": "02",
            "syntax": {
              "mnem": "FSTSW",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "w",
                "depend": "no",
                "#text": "AX"
              }
            },
            "grp1": "x87fpu",
            "grp2": "control",
            "modif_f_fpu": "0123",
            "undef_f_fpu": "0123",
            "note": {
              "brief": "Store x87 FPU Status Word"
            }
          },
          {
            "mod": "mem",
            "fpush": "yes",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "FILD",
              "dst": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              },
              "src": {
                "a": "M",
                "t": "qi"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Load Integer"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "5",
            "proc_start": "07",
            "syntax": {
              "mnem": "FUCOMIP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f": "ozpc",
            "def_f": "ozpc",
            "f_vals": "o",
            "modif_f_fpu": "1",
            "def_f_fpu": "1",
            "note": {
              "brief": "Unordered Compare Floating Point Values and Set EFLAGS and Pop"
            }
          },
          {
            "mod": "mem",
            "fpop": "once",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "FBSTP",
              "dst": {
                "a": "M",
                "t": "bcd"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store BCD Integer and Pop"
            }
          },
          {
            "mod": "nomem",
            "fpop": "once",
            "opcd_ext": "6",
            "proc_start": "07",
            "syntax": {
              "mnem": "FCOMIP",
              "src": [
                {
                  "nr": "0",
                  "group": "x87fpu",
                  "#text": "ST"
                },
                {
                  "a": "EST"
                }
              ]
            },
            "grp1": "x87fpu",
            "grp2": "compar",
            "modif_f": "ozpc",
            "def_f": "ozpc",
            "f_vals": "o",
            "modif_f_fpu": "1",
            "def_f_fpu": "1",
            "note": {
              "brief": "Compare Floating Point Values and Set EFLAGS and Pop"
            }
          },
          {
            "fpop": "once",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "FISTP",
              "dst": {
                "a": "M",
                "t": "qi"
              },
              "src": {
                "nr": "0",
                "group": "x87fpu",
                "displayed": "no",
                "#text": "ST"
              }
            },
            "grp1": "x87fpu",
            "grp2": "datamov",
            "modif_f_fpu": "0123",
            "def_f_fpu": "1",
            "undef_f_fpu": "023",
            "note": {
              "brief": "Store Integer and Pop"
            }
          }
        ]
      },
      {
        "value": "E0",
        "entry": [
          {
            "syntax": [
              {
                "mnem": "LOOPNZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              },
              {
                "mnem": "LOOPNE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "z",
            "note": {
              "brief": "Decrement count; Jump short if count!=0 and ZF=0"
            }
          },
          {
            "doc64_ref": "gen_note_short_near_jmp",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "LOOPNZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              },
              {
                "mnem": "LOOPNE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "z",
            "note": {
              "brief": "Decrement count; Jump short if count!=0 and ZF=0"
            }
          }
        ]
      },
      {
        "value": "E1",
        "entry": [
          {
            "syntax": [
              {
                "mnem": "LOOPZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              },
              {
                "mnem": "LOOPE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "z",
            "note": {
              "brief": "Decrement count; Jump short if count!=0 and ZF=1"
            }
          },
          {
            "doc64_ref": "gen_note_short_near_jmp",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "LOOPZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              },
              {
                "mnem": "LOOPE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                },
                "src": {
                  "a": "J",
                  "t": "bs"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "z",
            "note": {
              "brief": "Decrement count; Jump short if count!=0 and ZF=1"
            }
          }
        ]
      },
      {
        "value": "E2",
        "entry": [
          {
            "syntax": {
              "mnem": "LOOP",
              "dst": {
                "nr": "1",
                "group": "gen",
                "type": "va",
                "displayed": "no",
                "#text": "eCX"
              },
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Decrement count; Jump short if count!=0"
            }
          },
          {
            "doc64_ref": "gen_note_short_near_jmp",
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "LOOP",
              "dst": {
                "nr": "1",
                "group": "gen",
                "type": "dqa",
                "displayed": "no",
                "#text": "rCX"
              },
              "src": {
                "a": "J",
                "t": "bs"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Decrement count; Jump short if count!=0"
            }
          }
        ]
      },
      {
        "value": "E3",
        "entry": [
          {
            "syntax": [
              {
                "mnem": "JCXZ",
                "src": [
                  {
                    "a": "J",
                    "t": "bs"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "wa",
                    "displayed": "no",
                    "#text": "CX"
                  }
                ]
              },
              {
                "mnem": "JECXZ",
                "src": [
                  {
                    "a": "J",
                    "t": "bs"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "da",
                    "displayed": "no",
                    "#text": "ECX"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Jump short if eCX register is 0"
            }
          },
          {
            "doc64_ref": "gen_note_short_near_jmp",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "JECXZ",
                "src": [
                  {
                    "a": "J",
                    "t": "bs"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "da",
                    "displayed": "no",
                    "#text": "ECX"
                  }
                ]
              },
              {
                "mnem": "JRCXZ",
                "src": [
                  {
                    "a": "J",
                    "t": "bs"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "qa",
                    "displayed": "no",
                    "#text": "RCX"
                  }
                ]
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "note": {
              "brief": "Jump short if rCX register is 0"
            }
          }
        ]
      },
      {
        "value": "E4",
        "entry": {
          "op_size": "0",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "IN",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "depend": "no",
              "#text": "AL"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Input from Port"
          }
        }
      },
      {
        "value": "E5",
        "entry": {
          "op_size": "1",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "IN",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "v",
              "depend": "no",
              "#text": "eAX"
            },
            "src": {
              "a": "I",
              "t": "b"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Input from Port"
          }
        }
      },
      {
        "value": "E6",
        "entry": {
          "op_size": "0",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "OUT",
            "dst": {
              "a": "I",
              "t": "b"
            },
            "src": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Output to Port"
          }
        }
      },
      {
        "value": "E7",
        "entry": {
          "op_size": "1",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "OUT",
            "dst": {
              "a": "I",
              "t": "b"
            },
            "src": {
              "nr": "0",
              "group": "gen",
              "type": "v",
              "depend": "no",
              "#text": "eAX"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Output to Port"
          }
        }
      },
      {
        "value": "E8",
        "entry": {
          "doc64_ref": "gen_note_short_near_jmp",
          "syntax": {
            "mnem": "CALL",
            "dst": {
              "address": "SC",
              "displayed": "no",
              "#text": "SS:[rSP]"
            },
            "src": {
              "a": "J",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": [
            "branch",
            "stack"
          ],
          "note": {
            "brief": "Call Procedure"
          }
        }
      },
      {
        "value": "E9",
        "entry": {
          "doc64_ref": "gen_note_short_near_jmp",
          "syntax": {
            "mnem": "JMP",
            "src": {
              "a": "J",
              "t": "vds"
            }
          },
          "grp1": "gen",
          "grp2": "branch",
          "note": {
            "brief": "Jump"
          }
        }
      },
      {
        "value": "EA",
        "entry": [
          {
            "syntax": {
              "mnem": "JMPF",
              "src": {
                "a": "A",
                "t": "p"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "note": {
              "brief": "Jump"
            }
          },
          {
            "attr": "invd",
            "mode": "e",
            "proc_start": "10",
            "syntax": [],
            "note": {
              "brief": "Invalid Instruction in 64-Bit Mode"
            }
          }
        ]
      },
      {
        "value": "EB",
        "entry": {
          "syntax": {
            "mnem": "JMP",
            "src": {
              "a": "J",
              "t": "bs"
            }
          },
          "grp1": "gen",
          "grp2": "branch",
          "note": {
            "brief": "Jump"
          }
        }
      },
      {
        "value": "EC",
        "entry": {
          "op_size": "0",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "IN",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "depend": "no",
              "#text": "AL"
            },
            "src": {
              "nr": "2",
              "group": "gen",
              "type": "w",
              "#text": "DX"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Input from Port"
          }
        }
      },
      {
        "value": "ED",
        "entry": {
          "op_size": "1",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "IN",
            "dst": {
              "nr": "0",
              "group": "gen",
              "type": "v",
              "depend": "no",
              "#text": "eAX"
            },
            "src": {
              "nr": "2",
              "group": "gen",
              "type": "w",
              "#text": "DX"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Input from Port"
          }
        }
      },
      {
        "value": "EE",
        "entry": {
          "op_size": "0",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "OUT",
            "dst": {
              "nr": "2",
              "group": "gen",
              "type": "w",
              "#text": "DX"
            },
            "src": {
              "nr": "0",
              "group": "gen",
              "type": "b",
              "#text": "AL"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Output to Port"
          }
        }
      },
      {
        "value": "EF",
        "entry": {
          "op_size": "1",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "OUT",
            "dst": {
              "nr": "2",
              "group": "gen",
              "type": "w",
              "#text": "DX"
            },
            "src": {
              "nr": "0",
              "group": "gen",
              "type": "v",
              "depend": "no",
              "#text": "eAX"
            }
          },
          "grp1": "gen",
          "grp2": "inout",
          "note": {
            "brief": "Output to Port"
          }
        }
      },
      {
        "value": "F0",
        "entry": {
          "syntax": {
            "mnem": "LOCK"
          },
          "grp1": "prefix",
          "note": {
            "brief": "Assert LOCK# Signal Prefix"
          }
        }
      },
      {
        "value": "F1",
        "entry": [
          {
            "attr": "undef",
            "is_undoc": "yes",
            "doc_ref": "gen_note_undefined_D6_F1",
            "syntax": [],
            "note": {
              "brief": "Undefined and Reserved; Does not Generate #UD"
            }
          },
          {
            "part_alias": "CD",
            "doc_part_alias_ref": "gen_note_u_INT1_ICEBP_F1",
            "doc": "u",
            "is_doc": "yes",
            "doc_ref": "gen_note_u_INT1_ICEBP_F1",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "INT1",
                "dst": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                },
                "src": {
                  "type": "v",
                  "address": "F",
                  "displayed": "no",
                  "#text": "eFlags"
                }
              },
              {
                "mnem": "ICEBP",
                "dst": {
                  "address": "SC",
                  "displayed": "no",
                  "#text": "SS:[rSP]"
                },
                "src": {
                  "type": "v",
                  "address": "F",
                  "displayed": "no",
                  "#text": "eFlags"
                }
              }
            ],
            "grp1": "gen",
            "grp2": [
              "break",
              "stack"
            ],
            "modif_f": "i",
            "def_f": "i",
            "f_vals": "i",
            "note": {
              "brief": "Call to Interrupt Procedure"
            }
          }
        ]
      },
      {
        "value": "F2",
        "entry": [
          {
            "doc_ref": "gen_note_REP_F2_F3",
            "syntax": [
              {
                "mnem": "REPNZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                }
              },
              {
                "mnem": "REPNE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                }
              }
            ],
            "grp1": "prefix",
            "grp2": "string",
            "test_f": "z",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc": "u",
            "doc_ref": "gen_note_REP_F2_F3",
            "particular": "yes",
            "syntax": {
              "mnem": "REP",
              "dst": {
                "nr": "1",
                "group": "gen",
                "type": "va",
                "displayed": "no",
                "#text": "eCX"
              }
            },
            "grp1": "prefix",
            "grp2": "string",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc_ref": "gen_note_REP_F2_F3",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "REPNZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                }
              },
              {
                "mnem": "REPNE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                }
              }
            ],
            "grp1": "prefix",
            "grp2": "string",
            "test_f": "z",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc": "u",
            "doc_ref": "gen_note_REP_F2_F3",
            "mode": "e",
            "particular": "yes",
            "proc_start": "10",
            "syntax": {
              "mnem": "REP",
              "dst": {
                "nr": "1",
                "group": "gen",
                "type": "dqa",
                "displayed": "no",
                "#text": "rCX"
              }
            },
            "grp1": "prefix",
            "grp2": "string",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc": "m",
            "proc_start": "10",
            "syntax": [],
            "instr_ext": "sse2",
            "grp1": "prefix",
            "note": {
              "brief": "Scalar Double-precision Prefix"
            }
          }
        ]
      },
      {
        "value": "F3",
        "entry": [
          {
            "doc_ref": "gen_note_REP_F2_F3",
            "syntax": [
              {
                "mnem": "REPZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                }
              },
              {
                "mnem": "REPE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "va",
                  "displayed": "no",
                  "#text": "eCX"
                }
              }
            ],
            "grp1": "prefix",
            "grp2": "string",
            "test_f": "z",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc_ref": "gen_note_REP_F2_F3",
            "syntax": {
              "mnem": "REP",
              "dst": {
                "nr": "1",
                "group": "gen",
                "type": "va",
                "displayed": "no",
                "#text": "rCX"
              }
            },
            "grp1": "prefix",
            "grp2": "string",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc_ref": "gen_note_REP_F2_F3",
            "mode": "e",
            "proc_start": "10",
            "syntax": [
              {
                "mnem": "REPZ",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                }
              },
              {
                "mnem": "REPE",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqa",
                  "displayed": "no",
                  "#text": "rCX"
                }
              }
            ],
            "grp1": "prefix",
            "grp2": "string",
            "test_f": "z",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc_ref": "gen_note_REP_F2_F3",
            "mode": "e",
            "proc_start": "10",
            "syntax": {
              "mnem": "REP",
              "dst": {
                "nr": "1",
                "group": "gen",
                "type": "dqa",
                "displayed": "no",
                "#text": "rCX"
              }
            },
            "grp1": "prefix",
            "grp2": "string",
            "note": {
              "brief": "Repeat String Operation Prefix"
            }
          },
          {
            "doc": "m",
            "proc_start": "09",
            "syntax": [],
            "instr_ext": "sse1",
            "grp1": "prefix",
            "note": {
              "brief": "Scalar Single-precision Prefix"
            }
          }
        ]
      },
      {
        "value": "F4",
        "entry": {
          "ring": "0",
          "syntax": {
            "mnem": "HLT"
          },
          "grp1": "system",
          "note": {
            "brief": "Halt"
          }
        }
      },
      {
        "value": "F5",
        "entry": {
          "syntax": {
            "mnem": "CMC"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "test_f": "c",
          "modif_f": "c",
          "def_f": "c",
          "note": {
            "brief": "Complement Carry Flag"
          }
        }
      },
      {
        "value": "F6",
        "entry": [
          {
            "op_size": "0",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "TEST",
              "src": [
                {
                  "a": "E",
                  "t": "b"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Compare"
            }
          },
          {
            "op_size": "0",
            "alias": "F6_0",
            "doc": "u",
            "doc_ref": "gen_note_TEST_F6_1_F7_1",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "TEST",
              "src": [
                {
                  "a": "E",
                  "t": "b"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Compare"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "NOT",
              "dst": {
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "note": {
              "brief": "One's Complement Negation"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "NEG",
              "dst": {
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Two's Complement Negation"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "MUL",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "w",
                "displayed": "no",
                "#text": "AX"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "a": "E",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oc",
            "undef_f": "szap",
            "note": {
              "brief": "Unsigned Multiply"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "IMUL",
              "dst": {
                "nr": "0",
                "group": "gen",
                "type": "w",
                "displayed": "no",
                "#text": "AX"
              },
              "src": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "a": "E",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oc",
            "undef_f": "szap",
            "note": {
              "brief": "Signed Multiply"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "DIV",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AH"
                }
              ],
              "src": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "#text": "AX"
                },
                {
                  "a": "E",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "undef_f": "oszapc",
            "note": {
              "brief": "Unsigned Divide"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "IDIV",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AH"
                }
              ],
              "src": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "w",
                  "displayed": "no",
                  "#text": "AX"
                },
                {
                  "a": "E",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "undef_f": "oszapc",
            "note": {
              "brief": "Signed Divide"
            }
          }
        ]
      },
      {
        "value": "F7",
        "entry": [
          {
            "op_size": "1",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "TEST",
              "src": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "a": "I",
                  "t": "vqp"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Compare"
            }
          },
          {
            "op_size": "1",
            "alias": "F7_0",
            "doc": "u",
            "doc_ref": "gen_note_TEST_F6_1_F7_1",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "TEST",
              "src": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "a": "I",
                  "t": "vqp"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "logical",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "a",
            "f_vals": "oc",
            "note": {
              "brief": "Logical Compare"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "2",
            "syntax": {
              "mnem": "NOT",
              "dst": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "logical",
            "note": {
              "brief": "One's Complement Negation"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "NEG",
              "dst": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Two's Complement Negation"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "4",
            "syntax": {
              "mnem": "MUL",
              "dst": [
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "vqp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "rDX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rAX"
                }
              ],
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oc",
            "undef_f": "szap",
            "note": {
              "brief": "Unsigned Multiply"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "IMUL",
              "dst": [
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "vqp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "rDX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rAX"
                }
              ],
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oc",
            "undef_f": "szap",
            "note": {
              "brief": "Signed Multiply"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "6",
            "syntax": {
              "mnem": "DIV",
              "dst": [
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rDX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rAX"
                }
              ],
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "undef_f": "oszapc",
            "note": {
              "brief": "Unsigned Divide"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "7",
            "syntax": {
              "mnem": "IDIV",
              "dst": [
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rDX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rAX"
                }
              ],
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "undef_f": "oszapc",
            "note": {
              "brief": "Signed Divide"
            }
          }
        ]
      },
      {
        "value": "F8",
        "entry": {
          "syntax": {
            "mnem": "CLC"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "modif_f": "c",
          "def_f": "c",
          "f_vals": "c",
          "note": {
            "brief": "Clear Carry Flag"
          }
        }
      },
      {
        "value": "F9",
        "entry": {
          "syntax": {
            "mnem": "STC"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "modif_f": "c",
          "def_f": "c",
          "f_vals": "C",
          "note": {
            "brief": "Set Carry Flag"
          }
        }
      },
      {
        "value": "FA",
        "entry": {
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "CLI"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "modif_f": "i",
          "def_f": "i",
          "f_vals": "i",
          "note": {
            "brief": "Clear Interrupt Flag"
          }
        }
      },
      {
        "value": "FB",
        "entry": {
          "attr": "delaysint_cond",
          "ring": "f",
          "ring_ref": "rflags_iopl",
          "syntax": {
            "mnem": "STI"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "modif_f": "i",
          "def_f": "i",
          "f_vals": "I",
          "note": {
            "brief": "Set Interrupt Flag"
          }
        }
      },
      {
        "value": "FC",
        "entry": {
          "syntax": {
            "mnem": "CLD"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "modif_f": "d",
          "def_f": "d",
          "f_vals": "d",
          "note": {
            "brief": "Clear Direction Flag"
          }
        }
      },
      {
        "value": "FD",
        "entry": {
          "syntax": {
            "mnem": "STD"
          },
          "grp1": "gen",
          "grp2": "flgctrl",
          "modif_f": "d",
          "def_f": "d",
          "f_vals": "D",
          "note": {
            "brief": "Set Direction Flag"
          }
        }
      },
      {
        "value": "FE",
        "entry": [
          {
            "op_size": "0",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "INC",
              "dst": {
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszap",
            "def_f": "oszap",
            "note": {
              "brief": "Increment by 1"
            }
          },
          {
            "op_size": "0",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "DEC",
              "dst": {
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszap",
            "def_f": "oszap",
            "note": {
              "brief": "Decrement by 1"
            }
          }
        ]
      },
      {
        "value": "FF",
        "entry": [
          {
            "op_size": "1",
            "opcd_ext": "0",
            "syntax": {
              "mnem": "INC",
              "dst": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszap",
            "def_f": "oszap",
            "note": {
              "brief": "Increment by 1"
            }
          },
          {
            "op_size": "1",
            "opcd_ext": "1",
            "syntax": {
              "mnem": "DEC",
              "dst": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszap",
            "def_f": "oszap",
            "note": {
              "brief": "Decrement by 1"
            }
          },
          {
            "opcd_ext": "2",
            "syntax": {
              "mnem": "CALL",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": [
              "branch",
              "stack"
            ],
            "note": {
              "brief": "Call Procedure"
            }
          },
          {
            "doc64_ref": "gen_note_short_near_jmp",
            "mode": "e",
            "opcd_ext": "2",
            "proc_start": "10",
            "syntax": {
              "mnem": "CALL",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "E",
                "t": "q"
              }
            },
            "grp1": "gen",
            "grp2": [
              "branch",
              "stack"
            ],
            "note": {
              "brief": "Call Procedure"
            }
          },
          {
            "doc64_ref": "gen_note_CALLF_FF_3_JMPF_FF_5",
            "opcd_ext": "3",
            "syntax": {
              "mnem": "CALLF",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "M",
                "t": "ptp"
              }
            },
            "grp1": "gen",
            "grp2": [
              "branch",
              "stack"
            ],
            "note": {
              "brief": "Call Procedure"
            }
          },
          {
            "opcd_ext": "4",
            "syntax": {
              "mnem": "JMP",
              "src": {
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "note": {
              "brief": "Jump"
            }
          },
          {
            "doc64_ref": "gen_note_short_near_jmp",
            "mode": "e",
            "opcd_ext": "4",
            "proc_start": "10",
            "syntax": {
              "mnem": "JMP",
              "src": {
                "a": "E",
                "t": "q"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "note": {
              "brief": "Jump"
            }
          },
          {
            "doc64_ref": "gen_note_CALLF_FF_3_JMPF_FF_5",
            "opcd_ext": "5",
            "syntax": {
              "mnem": "JMPF",
              "src": {
                "a": "M",
                "t": "ptp"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "note": {
              "brief": "Jump"
            }
          },
          {
            "opcd_ext": "6",
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          },
          {
            "mode": "e",
            "opcd_ext": "6",
            "proc_start": "10",
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "a": "E",
                "t": "vq"
              }
            },
            "grp1": "gen",
            "grp2": "stack",
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          }
        ]
      }
    ],
    "two-byte": {
      "id": "two-byte",
      "pri_opcd": [
        {
          "value": "00",
          "proc_start": "02",
          "entry": [
            {
              "mode": "p",
              "opcd_ext": "0",
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "SLDT",
                  "dst": {
                    "depend": "no",
                    "a": "M",
                    "t": "w"
                  },
                  "src": {
                    "group": "systabp",
                    "displayed": "no",
                    "#text": "LDTR"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "SLDT",
                  "dst": {
                    "depend": "no",
                    "a": "R",
                    "t": "vqp"
                  },
                  "src": {
                    "group": "systabp",
                    "displayed": "no",
                    "#text": "LDTR"
                  }
                }
              ],
              "grp1": "system",
              "note": {
                "brief": "Store Local Descriptor Table Register"
              }
            },
            {
              "mode": "p",
              "opcd_ext": "1",
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "STR",
                  "dst": {
                    "depend": "no",
                    "a": "M",
                    "t": "w"
                  },
                  "src": {
                    "group": "systabp",
                    "displayed": "no",
                    "#text": "TR"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "STR",
                  "dst": {
                    "depend": "no",
                    "a": "R",
                    "t": "vqp"
                  },
                  "src": {
                    "group": "systabp",
                    "displayed": "no",
                    "#text": "TR"
                  }
                }
              ],
              "grp1": "system",
              "note": {
                "brief": "Store Task Register"
              }
            },
            {
              "attr": "serial",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "2",
              "syntax": {
                "mnem": "LLDT",
                "dst": {
                  "group": "systabp",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "LDTR"
                },
                "src": {
                  "a": "E",
                  "t": "w"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Load Local Descriptor Table Register"
              }
            },
            {
              "attr": "serial",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "3",
              "syntax": {
                "mnem": "LTR",
                "dst": {
                  "group": "systabp",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "TR"
                },
                "src": {
                  "a": "E",
                  "t": "w"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Load Task Register"
              }
            },
            {
              "mode": "p",
              "opcd_ext": "4",
              "syntax": {
                "mnem": "VERR",
                "src": {
                  "a": "E",
                  "t": "w"
                }
              },
              "grp1": "system",
              "modif_f": "z",
              "def_f": "z",
              "note": {
                "brief": "Verify a Segment for Reading"
              }
            },
            {
              "mode": "p",
              "opcd_ext": "5",
              "syntax": {
                "mnem": "VERW",
                "src": {
                  "a": "E",
                  "t": "w"
                }
              },
              "grp1": "system",
              "modif_f": "z",
              "def_f": "z",
              "note": {
                "brief": "Verify a Segment for Writing"
              }
            },
            {
              "opcd_ext": "6",
              "proc_start": "99",
              "syntax": {
                "mnem": "JMPE"
              },
              "grp1": "system",
              "grp2": "branch",
              "note": {
                "brief": "Jump to IA-64 Instruction Set"
              }
            }
          ]
        },
        {
          "value": "01",
          "proc_start": "02",
          "entry": [
            {
              "opcd_ext": "0",
              "syntax": {
                "mnem": "SGDT",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "s"
                },
                "src": {
                  "group": "systabp",
                  "displayed": "no",
                  "#text": "GDTR"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Store Global Descriptor Table Register"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "0",
              "sec_opcd": "C1",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMCALL"
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Call to VM Monitor",
                "det": "Call to VM monitor by causing VM exit"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "0",
              "sec_opcd": "C2",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMLAUNCH"
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Launch Virtual Machine",
                "det": "Launch virtual machine managed by current VMCS"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "0",
              "sec_opcd": "C3",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMRESUME"
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Resume Virtual Machine",
                "det": "Resume virtual machine managed by current VMCS"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "0",
              "sec_opcd": "C4",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMXOFF"
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Leave VMX Operation",
                "det": "Leaves VMX operation"
              }
            },
            {
              "opcd_ext": "1",
              "syntax": {
                "mnem": "SIDT",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "s"
                },
                "src": {
                  "group": "systabp",
                  "displayed": "no",
                  "#text": "IDTR"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Store Interrupt Descriptor Table Register"
              }
            },
            {
              "ring": "0",
              "opcd_ext": "1",
              "sec_opcd": "C8",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "MONITOR",
                "src": [
                  {
                    "type": "b",
                    "address": "BA",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "(DS:)[rAX]"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "ECX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  }
                ]
              },
              "instr_ext": "sse3",
              "grp1": "sync",
              "note": {
                "brief": "Set Up Monitor Address"
              }
            },
            {
              "ring": "0",
              "opcd_ext": "1",
              "sec_opcd": "C9",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "MWAIT",
                "src": [
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "ECX"
                  }
                ]
              },
              "instr_ext": "sse3",
              "grp1": "sync",
              "note": {
                "brief": "Monitor Wait"
              }
            },
            {
              "attr": "serial",
              "ring": "0",
              "opcd_ext": "2",
              "syntax": {
                "mnem": "LGDT",
                "dst": {
                  "group": "systabp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "GDTR"
                },
                "src": {
                  "a": "M",
                  "t": "s"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Load Global Descriptor Table Register"
              }
            },
            {
              "opcd_ext": "2",
              "sec_opcd": "D0",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "XGETBV",
                "dst": [
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EAX"
                  }
                ],
                "src": [
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "ECX"
                  },
                  {
                    "group": "xcr",
                    "displayed": "no",
                    "#text": "XCR"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Get Value of Extended Control Register",
                "det": "Reads an XCR specified by ECX into EDX:EAX"
              }
            },
            {
              "ring": "0",
              "opcd_ext": "2",
              "sec_opcd": "D1",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "XSETBV",
                "dst": {
                  "group": "xcr",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "XCR"
                },
                "src": [
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "ECX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Set Extended Control Register",
                "det": "Write the value in EDX:EAX to the XCR specified by ECX"
              }
            },
            {
              "attr": "serial",
              "ring": "0",
              "opcd_ext": "3",
              "syntax": {
                "mnem": "LIDT",
                "dst": {
                  "group": "systabp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "IDTR"
                },
                "src": {
                  "a": "M",
                  "t": "s"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Load Interrupt Descriptor Table Register"
              }
            },
            {
              "doc_ref": "gen_note_SMSW_0F01_4",
              "opcd_ext": "4",
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "SMSW",
                  "dst": {
                    "depend": "no",
                    "a": "M",
                    "t": "w"
                  },
                  "src": {
                    "nr": "0",
                    "group": "ctrl",
                    "type": "w",
                    "displayed": "no",
                    "#text": "MSW"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "SMSW",
                  "dst": {
                    "depend": "no",
                    "a": "R",
                    "t": "vqp"
                  },
                  "src": {
                    "nr": "0",
                    "group": "ctrl",
                    "type": "w",
                    "displayed": "no",
                    "#text": "MSW"
                  }
                }
              ],
              "grp1": "system",
              "note": {
                "brief": "Store Machine Status Word"
              }
            },
            {
              "attr": "serial",
              "ring": "0",
              "opcd_ext": "6",
              "syntax": {
                "mnem": "LMSW",
                "dst": {
                  "nr": "0",
                  "group": "ctrl",
                  "type": "w",
                  "displayed": "no",
                  "depend": "no",
                  "#text": "MSW"
                },
                "src": {
                  "a": "E",
                  "t": "w"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Load Machine Status Word"
              }
            },
            {
              "attr": "serial",
              "ring": "0",
              "opcd_ext": "7",
              "proc_start": "04",
              "syntax": {
                "mnem": "INVLPG",
                "src": {
                  "depend": "no",
                  "a": "M"
                }
              },
              "grp1": "system",
              "note": {
                "brief": "Invalidate TLB Entry"
              }
            },
            {
              "ring": "0",
              "mode": "e",
              "opcd_ext": "7",
              "sec_opcd": "F8",
              "proc_start": "10",
              "syntax": {
                "mnem": "SWAPGS",
                "dst": [
                  {
                    "nr": "5",
                    "group": "seg",
                    "type": "w",
                    "displayed": "no",
                    "#text": "GS"
                  },
                  {
                    "nr": "C0000102",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_KERNEL_GSBASE"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Swap GS Base Register"
              }
            },
            {
              "attr": "serial",
              "ring": "f",
              "ring_ref": "cr4_tsd",
              "opcd_ext": "7",
              "sec_opcd": "F9",
              "proc_start": "13",
              "syntax": {
                "mnem": "RDTSCP",
                "dst": [
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EAX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ECX"
                  }
                ],
                "src": [
                  {
                    "nr": "10",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_TIME_STAMP_COUNTER"
                  },
                  {
                    "nr": "C0000103",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_TSC_AUX"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Read Time-Stamp Counter and Processor ID",
                "det": "Read 64-bit time-stamp counter and 32-bit IA32_TSC_AUX value into EDX:EAX and ECX"
              }
            }
          ]
        },
        {
          "value": "02",
          "entry": {
            "r": "yes",
            "mode": "p",
            "proc_start": "02",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "LAR",
                "dst": {
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "M",
                  "t": "w"
                }
              },
              {
                "mod": "nomem",
                "mnem": "LAR",
                "dst": {
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "R",
                  "t": "v"
                }
              }
            ],
            "grp1": "system",
            "modif_f": "z",
            "def_f": "z",
            "note": {
              "brief": "Load Access Rights Byte"
            }
          }
        },
        {
          "value": "03",
          "entry": {
            "r": "yes",
            "mode": "p",
            "proc_start": "02",
            "syntax": [
              {
                "mod": "mem",
                "mnem": "LSL",
                "dst": {
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "M",
                  "t": "w"
                }
              },
              {
                "mod": "nomem",
                "mnem": "LSL",
                "dst": {
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "R",
                  "t": "v"
                }
              }
            ],
            "grp1": "system",
            "modif_f": "z",
            "def_f": "z",
            "note": {
              "brief": "Load Segment Limit"
            }
          }
        },
        {
          "value": "05",
          "entry": [
            {
              "doc": "u",
              "doc1632_ref": "gen_note_u_LOADALL_0F05_0F07",
              "mode": "p",
              "particular": "yes",
              "proc_start": {
                "post": "no",
                "#text": "02"
              },
              "proc_end": "02",
              "syntax": {
                "mnem": "LOADALL",
                "dst": [
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "AX"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "CX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "DX"
                  },
                  {
                    "nr": "3",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "BX"
                  },
                  {
                    "nr": "4",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "SP"
                  },
                  {
                    "nr": "5",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "BP"
                  },
                  {
                    "nr": "6",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "SI"
                  },
                  {
                    "nr": "7",
                    "group": "gen",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "DI"
                  },
                  {
                    "type": "w",
                    "address": "F",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "Flags"
                  },
                  {
                    "nr": "0",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ES"
                  },
                  {
                    "nr": "2",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "SS"
                  },
                  {
                    "nr": "3",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "DS"
                  },
                  {
                    "nr": "0",
                    "group": "ctrl",
                    "type": "w",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "MSW"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "TR"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "LDTR"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "GDTR"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "IDTR"
                  }
                ]
              },
              "grp1": "system",
              "grp2": "branch",
              "note": {
                "brief": "Load All of the CPU Registers"
              }
            },
            {
              "doc64_ref": "gen_note_SYSCALL_0F05",
              "mode": "e",
              "proc_start": "10",
              "syntax": {
                "mnem": "SYSCALL",
                "dst": [
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "q",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "RCX"
                  },
                  {
                    "nr": "11",
                    "group": "gen",
                    "type": "q",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "R11"
                  },
                  {
                    "nr": "2",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "SS"
                  }
                ],
                "src": [
                  {
                    "type": "d",
                    "address": "F",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EFlags"
                  },
                  {
                    "nr": "C0000082",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_LSTAR"
                  },
                  {
                    "nr": "C0000084",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_FMASK"
                  }
                ]
              },
              "grp1": "system",
              "grp2": "branch",
              "note": {
                "brief": "Fast System Call"
              }
            }
          ]
        },
        {
          "value": "06",
          "entry": {
            "ring": "0",
            "proc_start": "02",
            "syntax": {
              "mnem": "CLTS",
              "dst": {
                "nr": "0",
                "group": "ctrl",
                "displayed": "no",
                "#text": "CR0"
              }
            },
            "grp1": "system",
            "note": {
              "brief": "Clear Task-Switched Flag in CR0"
            }
          }
        },
        {
          "value": "07",
          "entry": [
            {
              "doc": "u",
              "doc1632_ref": "gen_note_u_LOADALL_0F05_0F07",
              "mode": "p",
              "particular": "yes",
              "proc_start": {
                "post": "no",
                "#text": "03"
              },
              "proc_end": "03",
              "syntax": {
                "mnem": "LOADALL",
                "dst": [
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EAX"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ECX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "3",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EBX"
                  },
                  {
                    "nr": "4",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ESP"
                  },
                  {
                    "nr": "5",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EBP"
                  },
                  {
                    "nr": "6",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ESI"
                  },
                  {
                    "nr": "7",
                    "group": "gen",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EDI"
                  },
                  {
                    "type": "d",
                    "address": "F",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EFlags"
                  },
                  {
                    "nr": "0",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ES"
                  },
                  {
                    "nr": "2",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "SS"
                  },
                  {
                    "nr": "3",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "DS"
                  },
                  {
                    "nr": "4",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "FS"
                  },
                  {
                    "nr": "5",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "GS"
                  },
                  {
                    "nr": "0",
                    "group": "ctrl",
                    "type": "d",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "CR0"
                  },
                  {
                    "nr": "6",
                    "group": "debug",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "DR6"
                  },
                  {
                    "nr": "7",
                    "group": "debug",
                    "type": "d",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "DR7"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "TR"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "LDTR"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "GDTR"
                  },
                  {
                    "group": "systabp",
                    "displayed": "no",
                    "depend": "no",
                    "#text": "IDTR"
                  }
                ]
              },
              "grp1": "system",
              "grp2": "branch",
              "note": {
                "brief": "Load All of the CPU Registers"
              }
            },
            {
              "ring": "0",
              "mode": "e",
              "proc_start": "10",
              "syntax": {
                "mnem": "SYSRET",
                "dst": [
                  {
                    "nr": "2",
                    "group": "seg",
                    "type": "w",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "SS"
                  },
                  {
                    "type": "d",
                    "address": "F",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "EFlags"
                  }
                ],
                "src": [
                  {
                    "nr": "11",
                    "group": "gen",
                    "type": "q",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "R11"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "q",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "RCX"
                  },
                  {
                    "nr": "C0000081",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_STAR"
                  }
                ]
              },
              "grp1": "system",
              "grp2": "branch",
              "grp3": "trans",
              "note": {
                "brief": "Return From Fast System Call"
              }
            }
          ]
        },
        {
          "value": "08",
          "entry": {
            "attr": "serial",
            "ring": "0",
            "proc_start": "04",
            "syntax": {
              "mnem": "INVD"
            },
            "grp1": "system",
            "note": {
              "brief": "Invalidate Internal Caches"
            }
          }
        },
        {
          "value": "09",
          "entry": {
            "attr": "serial",
            "ring": "0",
            "proc_start": "04",
            "syntax": {
              "mnem": "WBINVD"
            },
            "grp1": "system",
            "note": {
              "brief": "Write Back and Invalidate Cache"
            }
          }
        },
        {
          "value": "0B",
          "entry": {
            "attr": "invd",
            "proc_start": "02",
            "syntax": {
              "mnem": "UD2"
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Undefined Instruction"
            }
          }
        },
        {
          "value": "0D",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_NOP_0F0D",
            "proc_start": "07",
            "syntax": {
              "mnem": "NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "No Operation"
            }
          }
        },
        {
          "value": "10",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVUPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Unaligned Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVSS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVUPD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Unaligned Packed Double-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVSD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Scalar Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "11",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVUPS",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "ps"
                },
                "src": {
                  "a": "V",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Unaligned Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVSS",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "ss"
                },
                "src": {
                  "a": "V",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVUPD",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "pd"
                },
                "src": {
                  "a": "V",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Unaligned Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVSD",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "sd"
                },
                "src": {
                  "a": "V",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Scalar Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "12",
          "entry": [
            {
              "r": "yes",
              "mod": "nomem",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVHLPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "U",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Packed Single-FP Values High to Low"
              }
            },
            {
              "r": "yes",
              "mod": "mem",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVLPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Low Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVLPD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Low Packed Double-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "MOVDDUP",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "W",
                  "t": "q"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move One Double-FP and Duplicate"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "MOVSLDUP",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "W",
                  "t": "q"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Packed Single-FP Low and Duplicate"
              }
            }
          ]
        },
        {
          "value": "13",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVLPS",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "q"
                },
                "src": {
                  "a": "V",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Low Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVLPD",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "q"
                },
                "src": {
                  "a": "V",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Low Packed Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "14",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "UNPCKLPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack and Interleave Low Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "UNPCKLPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack and Interleave Low Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "15",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "UNPCKHPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack and Interleave High Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "UNPCKHPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack and Interleave High Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "16",
          "entry": [
            {
              "r": "yes",
              "mod": "nomem",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVLHPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "U",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Packed Single-FP Values Low to High"
              }
            },
            {
              "r": "yes",
              "mod": "mem",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVHPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move High Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVHPD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move High Packed Double-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "MOVSHDUP",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "W",
                  "t": "q"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Packed Single-FP High and Duplicate"
              }
            }
          ]
        },
        {
          "value": "17",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVHPS",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "q"
                },
                "src": {
                  "a": "V",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move High Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVHPD",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "q"
                },
                "src": {
                  "a": "V",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move High Packed Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "18",
          "entry": [
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "proc_start": "07",
              "proc_end": "08",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "opcd_ext": "0",
              "proc_start": "09",
              "syntax": {
                "mnem": "PREFETCHNTA",
                "src": {
                  "depend": "no",
                  "a": "M",
                  "t": "b"
                }
              },
              "instr_ext": "sse1",
              "grp1": "fetch",
              "note": {
                "brief": "Prefetch Data Into Caches"
              }
            },
            {
              "opcd_ext": "1",
              "proc_start": "09",
              "syntax": {
                "mnem": "PREFETCHT0",
                "src": {
                  "depend": "no",
                  "a": "M",
                  "t": "b"
                }
              },
              "instr_ext": "sse1",
              "grp1": "fetch",
              "note": {
                "brief": "Prefetch Data Into Caches"
              }
            },
            {
              "opcd_ext": "2",
              "proc_start": "09",
              "syntax": {
                "mnem": "PREFETCHT1",
                "src": {
                  "depend": "no",
                  "a": "M",
                  "t": "b"
                }
              },
              "instr_ext": "sse1",
              "grp1": "fetch",
              "note": {
                "brief": "Prefetch Data Into Caches"
              }
            },
            {
              "opcd_ext": "3",
              "proc_start": "09",
              "syntax": {
                "mnem": "PREFETCHT2",
                "src": {
                  "depend": "no",
                  "a": "M",
                  "t": "b"
                }
              },
              "instr_ext": "sse1",
              "grp1": "fetch",
              "note": {
                "brief": "Prefetch Data Into Caches"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "4",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "5",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "6",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "7",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            }
          ]
        },
        {
          "value": "19",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
            "proc_start": "07",
            "syntax": {
              "mnem": "HINT_NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Hintable NOP"
            }
          }
        },
        {
          "value": "1A",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
            "proc_start": "07",
            "syntax": {
              "mnem": "HINT_NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Hintable NOP"
            }
          }
        },
        {
          "value": "1B",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
            "proc_start": "07",
            "syntax": {
              "mnem": "HINT_NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Hintable NOP"
            }
          }
        },
        {
          "value": "1C",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
            "proc_start": "07",
            "syntax": {
              "mnem": "HINT_NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Hintable NOP"
            }
          }
        },
        {
          "value": "1D",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
            "proc_start": "07",
            "syntax": {
              "mnem": "HINT_NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Hintable NOP"
            }
          }
        },
        {
          "value": "1E",
          "entry": {
            "doc": "m",
            "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
            "proc_start": "07",
            "syntax": {
              "mnem": "HINT_NOP",
              "src": {
                "depend": "no",
                "a": "E",
                "t": "v"
              }
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Hintable NOP"
            }
          }
        },
        {
          "value": "1F",
          "entry": [
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "proc_start": "07",
              "proc_end": "08",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "opcd_ext": "0",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "No Operation"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "1",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "2",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "3",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "4",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "5",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "6",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            },
            {
              "doc": "m",
              "doc_ref": "gen_note_hintable_nop_0F18_0F1F",
              "opcd_ext": "7",
              "proc_start": "07",
              "syntax": {
                "mnem": "HINT_NOP",
                "src": {
                  "depend": "no",
                  "a": "E",
                  "t": "v"
                }
              },
              "grp1": "gen",
              "grp2": "control",
              "note": {
                "brief": "Hintable NOP"
              }
            }
          ]
        },
        {
          "value": "20",
          "entry": [
            {
              "r": "yes",
              "is_undoc": "yes",
              "doc64_ref": "gen_note_MOV_CR_0F20_0F22",
              "ring": "0",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "R",
                  "t": "d"
                },
                "src": {
                  "a": "C",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "particular": "yes",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "H",
                  "t": "d"
                },
                "src": {
                  "a": "C",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            },
            {
              "r": "yes",
              "is_undoc": "yes",
              "ring": "0",
              "mode": "e",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "R",
                  "t": "q"
                },
                "src": {
                  "a": "C",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "mode": "e",
              "particular": "yes",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "H",
                  "t": "q"
                },
                "src": {
                  "a": "C",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            }
          ]
        },
        {
          "value": "21",
          "entry": [
            {
              "r": "yes",
              "is_undoc": "yes",
              "ring": "0",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "R",
                  "t": "d"
                },
                "src": {
                  "a": "D",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "particular": "yes",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "H",
                  "t": "d"
                },
                "src": {
                  "a": "D",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            },
            {
              "r": "yes",
              "is_undoc": "yes",
              "ring": "0",
              "mode": "e",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "R",
                  "t": "q"
                },
                "src": {
                  "a": "D",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "mode": "e",
              "particular": "yes",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "H",
                  "t": "q"
                },
                "src": {
                  "a": "D",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            }
          ]
        },
        {
          "value": "22",
          "entry": [
            {
              "r": "yes",
              "attr": "serial",
              "is_undoc": "yes",
              "doc64_ref": "gen_note_MOV_CR_0F20_0F22",
              "ring": "0",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "C",
                  "t": "d"
                },
                "src": {
                  "a": "R",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            },
            {
              "r": "yes",
              "attr": "serial",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "particular": "yes",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "C",
                  "t": "d"
                },
                "src": {
                  "a": "H",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            },
            {
              "r": "yes",
              "is_undoc": "yes",
              "ring": "0",
              "mode": "e",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "C",
                  "t": "q"
                },
                "src": {
                  "a": "R",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "mode": "e",
              "particular": "yes",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "C",
                  "t": "q"
                },
                "src": {
                  "a": "H",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Control Registers"
              }
            }
          ]
        },
        {
          "value": "23",
          "entry": [
            {
              "r": "yes",
              "attr": "serial",
              "is_undoc": "yes",
              "ring": "0",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "D",
                  "t": "d"
                },
                "src": {
                  "a": "R",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            },
            {
              "r": "yes",
              "attr": "serial",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "particular": "yes",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "D",
                  "t": "q"
                },
                "src": {
                  "a": "H",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            },
            {
              "r": "yes",
              "attr": "serial",
              "is_undoc": "yes",
              "ring": "0",
              "mode": "e",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "D",
                  "t": "q"
                },
                "src": {
                  "a": "R",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            },
            {
              "r": "yes",
              "attr": "serial",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "mode": "e",
              "particular": "yes",
              "proc_start": "03",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "D",
                  "t": "q"
                },
                "src": {
                  "a": "H",
                  "t": "q"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Debug Registers"
              }
            }
          ]
        },
        {
          "value": "24",
          "entry": [
            {
              "r": "yes",
              "is_undoc": "yes",
              "ring": "0",
              "proc_start": "03",
              "proc_end": "04",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "R",
                  "t": "d"
                },
                "src": {
                  "a": "T",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Test Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "particular": "yes",
              "proc_start": "03",
              "proc_end": "04",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "H",
                  "t": "d"
                },
                "src": {
                  "a": "T",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Test Registers"
              }
            }
          ]
        },
        {
          "value": "26",
          "entry": [
            {
              "r": "yes",
              "is_undoc": "yes",
              "ring": "0",
              "proc_start": "03",
              "proc_end": "04",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "T",
                  "t": "d"
                },
                "src": {
                  "a": "R",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Test Registers"
              }
            },
            {
              "r": "yes",
              "doc": "u",
              "is_doc": "yes",
              "doc_ref": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26",
              "ring": "0",
              "particular": "yes",
              "proc_start": "03",
              "proc_end": "04",
              "syntax": {
                "mnem": "MOV",
                "dst": {
                  "depend": "no",
                  "a": "T",
                  "t": "d"
                },
                "src": {
                  "a": "H",
                  "t": "d"
                }
              },
              "grp1": "system",
              "modif_f": "oszapc",
              "undef_f": "oszapc",
              "note": {
                "brief": "Move to/from Test Registers"
              }
            }
          ]
        },
        {
          "value": "28",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVAPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Aligned Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVAPD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Aligned Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "29",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVAPS",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "ps"
                },
                "src": {
                  "a": "V",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Move Aligned Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVAPD",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "pd"
                },
                "src": {
                  "a": "V",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Move Aligned Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "2A",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "CVTPI2PS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "Q",
                  "t": "pi"
                }
              },
              "instr_ext": "sse1",
              "grp1": "conver",
              "note": {
                "brief": [
                  "Convert Packed DW Integers to",
                  "Single-FP Values"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "CVTSI2SS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "E",
                  "t": "dqp"
                }
              },
              "instr_ext": "sse1",
              "grp1": "conver",
              "note": {
                "brief": "Convert DW Integer to Scalar Single-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTPI2PD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "Q",
                  "t": "pi"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert Packed DW Integers to",
                  "Double-FP Values"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTSI2SD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "E",
                  "t": "dqp"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": "Convert DW Integer to Scalar Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "2B",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVNTPS",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "ps"
                },
                "src": {
                  "a": "V",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "cachect",
              "note": {
                "brief": "Store Packed Single-FP Values Using Non-Temporal Hint"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVNTPD",
                "dst": {
                  "a": "M",
                  "t": "pd"
                },
                "src": {
                  "a": "V",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "cachect",
              "note": {
                "brief": "Store Packed Double-FP Values Using Non-Temporal Hint"
              }
            }
          ]
        },
        {
          "value": "2C",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "CVTTPS2PI",
                "dst": {
                  "a": "P",
                  "t": "pi"
                },
                "src": {
                  "a": "W",
                  "t": "psq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "conver",
              "note": {
                "brief": [
                  "Convert with Trunc. Packed Single-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "CVTTSS2SI",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "conver",
              "note": {
                "brief": "Convert with Trunc. Scalar Single-FP Value to DW Integer"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTTPD2PI",
                "dst": {
                  "a": "P",
                  "t": "pi"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert with Trunc. Packed Double-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTTSD2SI",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": "Conv. with Trunc. Scalar Double-FP Value to Signed DW Int"
              }
            }
          ]
        },
        {
          "value": "2D",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "CVTPS2PI",
                "dst": {
                  "a": "P",
                  "t": "pi"
                },
                "src": {
                  "a": "W",
                  "t": "psq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "conver",
              "note": {
                "brief": [
                  "Convert Packed Single-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "CVTSS2SI",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "conver",
              "note": {
                "brief": "Convert Scalar Single-FP Value to DW Integer"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTPD2PI",
                "dst": {
                  "a": "P",
                  "t": "pi"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert Packed Double-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTSD2SI",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": "Convert Scalar Double-FP Value to DW Integer"
              }
            }
          ]
        },
        {
          "value": "2E",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "UCOMISS",
                "src": [
                  {
                    "a": "V",
                    "t": "ss"
                  },
                  {
                    "a": "W",
                    "t": "ss"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "compar",
              "modif_f": "zpc",
              "def_f": "zpc",
              "note": {
                "brief": "Unordered Compare Scalar Single-FP Values and Set EFLAGS"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "UCOMISD",
                "src": [
                  {
                    "a": "V",
                    "t": "sd"
                  },
                  {
                    "a": "W",
                    "t": "sd"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "compar",
              "modif_f": "zpc",
              "def_f": "zpc",
              "note": {
                "brief": "Unordered Compare Scalar Double-FP Values and Set EFLAGS"
              }
            }
          ]
        },
        {
          "value": "2F",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "COMISS",
                "src": [
                  {
                    "a": "V",
                    "t": "ss"
                  },
                  {
                    "a": "W",
                    "t": "ss"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "compar",
              "modif_f": "zpc",
              "def_f": "zpc",
              "note": {
                "brief": "Compare Scalar Ordered Single-FP Values and Set EFLAGS"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "COMISD",
                "src": [
                  {
                    "a": "V",
                    "t": "sd"
                  },
                  {
                    "a": "W",
                    "t": "sd"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "compar",
              "modif_f": "zpc",
              "def_f": "zpc",
              "note": {
                "brief": "Compare Scalar Ordered Double-FP Values and Set EFLAGS"
              }
            }
          ]
        },
        {
          "value": "30",
          "entry": {
            "attr": "serial",
            "ring": "0",
            "proc_start": "05",
            "syntax": {
              "mnem": "WRMSR",
              "dst": {
                "group": "msr",
                "depend": "no",
                "displayed": "no",
                "#text": "MSR"
              },
              "src": [
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rCX"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rAX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rDX"
                }
              ]
            },
            "grp1": "system",
            "note": {
              "brief": "Write to Model Specific Register"
            }
          }
        },
        {
          "value": "31",
          "entry": {
            "ring": "f",
            "ring_ref": "cr4_tsd",
            "proc_start": "05",
            "syntax": {
              "mnem": "RDTSC",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EAX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EDX"
                }
              ],
              "src": {
                "nr": "10",
                "group": "msr",
                "displayed": "no",
                "#text": "IA32_TIME_STAMP_COUNTER"
              }
            },
            "grp1": "system",
            "note": {
              "brief": "Read Time-Stamp Counter"
            }
          }
        },
        {
          "value": "32",
          "entry": {
            "ring": "0",
            "proc_start": "05",
            "syntax": {
              "mnem": "RDMSR",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "dqp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "rAX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "dqp",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "rDX"
                }
              ],
              "src": [
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rCX"
                },
                {
                  "group": "msr",
                  "displayed": "no",
                  "#text": "MSR"
                }
              ]
            },
            "grp1": "system",
            "note": {
              "brief": "Read from Model Specific Register"
            }
          }
        },
        {
          "value": "33",
          "entry": {
            "ring": "f",
            "ring_ref": "cr4_pce",
            "proc_start": "06",
            "syntax": {
              "mnem": "RDPMC",
              "dst": [
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EAX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EDX"
                }
              ],
              "src": {
                "group": "msr",
                "displayed": "no",
                "#text": "PMC"
              }
            },
            "grp1": "system",
            "note": {
              "brief": "Read Performance-Monitoring Counters"
            }
          }
        },
        {
          "value": "34",
          "entry": [
            {
              "mode": "p",
              "proc_start": "08",
              "syntax": {
                "mnem": "SYSENTER",
                "dst": [
                  {
                    "nr": "2",
                    "group": "seg",
                    "type": "w",
                    "address": "S2",
                    "displayed": "no",
                    "#text": "SS"
                  },
                  {
                    "nr": "4",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "ESP"
                  }
                ],
                "src": [
                  {
                    "nr": "174",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_SYSENTER_CS"
                  },
                  {
                    "nr": "175",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_SYSENTER_ESP"
                  },
                  {
                    "nr": "176",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_SYSENTER_EIP"
                  }
                ]
              },
              "grp1": "system",
              "grp2": "branch",
              "modif_f": "i",
              "def_f": "i",
              "f_vals": "i",
              "note": {
                "brief": "Fast System Call"
              }
            },
            {
              "doc_ref": "gen_note_SYSENTER_0F34",
              "mode": "e",
              "proc_start": "10",
              "syntax": {
                "mnem": "SYSENTER",
                "dst": [
                  {
                    "nr": "2",
                    "group": "seg",
                    "type": "w",
                    "address": "S2",
                    "displayed": "no",
                    "#text": "SS"
                  },
                  {
                    "nr": "4",
                    "group": "gen",
                    "type": "q",
                    "displayed": "no",
                    "#text": "RSP"
                  }
                ],
                "src": [
                  {
                    "nr": "174",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_SYSENTER_CS"
                  },
                  {
                    "nr": "175",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_SYSENTER_ESP"
                  },
                  {
                    "nr": "176",
                    "group": "msr",
                    "displayed": "no",
                    "#text": "IA32_SYSENTER_EIP"
                  }
                ]
              },
              "grp1": "system",
              "grp2": "branch",
              "modif_f": "i",
              "def_f": "i",
              "f_vals": "i",
              "note": {
                "brief": "Fast System Call"
              }
            }
          ]
        },
        {
          "value": "35",
          "entry": {
            "doc64_ref": "gen_note_SYSEXIT_0F35",
            "ring": "0",
            "mode": "p",
            "proc_start": "08",
            "syntax": {
              "mnem": "SYSEXIT",
              "dst": [
                {
                  "nr": "2",
                  "group": "seg",
                  "type": "w",
                  "address": "S2",
                  "displayed": "no",
                  "#text": "SS"
                },
                {
                  "nr": "4",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "eSP"
                }
              ],
              "src": [
                {
                  "nr": "174",
                  "group": "msr",
                  "displayed": "no",
                  "#text": "IA32_SYSENTER_CS"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rCX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rDX"
                }
              ]
            },
            "grp1": "system",
            "grp2": "branch",
            "grp3": "trans",
            "note": {
              "brief": "Fast Return from Fast System Call"
            }
          }
        },
        {
          "value": "37",
          "entry": {
            "doc_ref": "gen_note_GETSEC_0F37",
            "proc_start": {
              "lat_step": "yes",
              "#text": "12"
            },
            "syntax": {
              "mnem": "GETSEC",
              "src": {
                "nr": "0",
                "group": "gen",
                "type": "d",
                "displayed": "no",
                "#text": "EAX"
              }
            },
            "instr_ext": "smx",
            "note": {
              "brief": "GETSEC Leaf Functions"
            }
          }
        },
        {
          "value": "38",
          "entry": [
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "00"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSHUFB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Shuffle Bytes"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "00"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSHUFB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Shuffle Bytes"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "01"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHADDW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Add"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "01"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHADDW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Add"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "02"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHADDD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Add"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "02"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHADDD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Add"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "03"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHADDSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Add and Saturate"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "03"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHADDSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Add and Saturate"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "04"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PMADDUBSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Multiply and Add Packed Signed and Unsigned Bytes"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "04"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PMADDUBSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Multiply and Add Packed Signed and Unsigned Bytes"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "05"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHSUBW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Subtract"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "05"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHSUBW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Subtract"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "06"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHSUBD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Subtract"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "06"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHSUBD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Subtract"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "07"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHSUBSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Subtract and Saturate"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "07"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PHSUBSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Horizontal Subtract and Saturate"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "08"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSIGNB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed SIGN"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "08"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSIGNB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed SIGN"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "09"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSIGNW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed SIGN"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "09"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSIGNW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed SIGN"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0A"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSIGND",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed SIGN"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0A"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PSIGND",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed SIGN"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0B"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PMULHRSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Multiply High with Round and Scale"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0B"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PMULHRSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Multiply High with Round and Scale"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "10"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PBLENDVB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Variable Blend Packed Bytes"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "14"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "BLENDVPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ps"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Variable Blend Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "15"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "BLENDVPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "pd"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Variable Blend Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "17"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PTEST",
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "W",
                    "t": "dq"
                  }
                ]
              },
              "instr_ext": "sse41",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "f_vals": "osap",
              "note": {
                "brief": "Logical Compare"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "1C"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PABSB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Absolute Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "1C"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PABSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Absolute Value"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "1D"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PABSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Absolute Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "1D"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PABSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Absolute Value"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "1E"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PABSD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Absolute Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "1E"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PABSD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Absolute Value"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "20"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVSXBW",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "q"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVSXBW",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Sign Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "21"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVSXBD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "d"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVSXBD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Sign Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "22"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVSXBQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "w"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVSXBQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Sign Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "23"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVSXWD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "q"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVSXWD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Sign Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "24"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVSXWQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "d"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVSXWQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Sign Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "25"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVSXDQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "q"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVSXDQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Sign Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "28"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMULDQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Signed Dword Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "29"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PCMPEQQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Qword Data for Equal"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "2A"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "MOVNTDQA",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "M",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "cachect",
              "note": {
                "brief": "Load Double Quadword Non-Temporal Aligned Hint"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "2B"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PACKUSDW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Pack with Unsigned Saturation"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "30"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVZXBW",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "q"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVZXBW",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Zero Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "31"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVZXBD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "d"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVZXBD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Zero Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "32"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVZXBQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "w"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVZXBQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Zero Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "33"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVZXWD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "q"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVZXWD",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Zero Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "34"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVZXWQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "d"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVZXWQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Zero Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "35"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PMOVZXDQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "M",
                    "t": "q"
                  }
                },
                {
                  "mod": "nomem",
                  "mnem": "PMOVZXDQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "U",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Packed Move with Zero Extend"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "37"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PCMPGTQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse42",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Qword Data for Greater Than",
                "det": "Compare packed qwords in xmm2/m128 and xmm1 for greater than"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "38"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMINSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Minimum of Packed Signed Byte Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "39"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMINSD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Minimum of Packed Signed Dword Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "3A"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMINUW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Minimum of Packed Unsigned Word Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "3B"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMINUD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Minimum of Packed Unsigned Dword Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "3C"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMAXSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Maximum of Packed Signed Byte Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "3D"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMAXSD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Maximum of Packed Signed Dword Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "3E"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMAXUW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Maximum of Packed Unsigned Word Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "3F"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMAXUD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Maximum of Packed Unsigned Dword Integers"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "40"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PMULLD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Signed Dword Integers and Store Low Result"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "41"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PHMINPOSUW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Packed Horizontal Word Minimum"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "80"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "INVEPT",
                "src": [
                  {
                    "a": "G",
                    "t": "d"
                  },
                  {
                    "a": "M",
                    "t": "dq"
                  }
                ]
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Invalidate Translations Derived from EPT",
                "det": "Invalidates EPT-derived entries in the TLBs and paging-structure caches"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "e",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "80"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "INVEPT",
                "src": [
                  {
                    "a": "G",
                    "t": "q"
                  },
                  {
                    "a": "M",
                    "t": "dq"
                  }
                ]
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Invalidate Translations Derived from EPT",
                "det": "Invalidates EPT-derived entries in the TLBs and paging-structure caches"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "81"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "INVVPID",
                "src": [
                  {
                    "a": "G",
                    "t": "d"
                  },
                  {
                    "a": "M",
                    "t": "dq"
                  }
                ]
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Invalidate Translations Based on VPID",
                "det": "Invalidates entries in the TLBs and paging-structure caches based on VPID"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "e",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "81"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "INVVPID",
                "src": [
                  {
                    "a": "G",
                    "t": "q"
                  },
                  {
                    "a": "M",
                    "t": "dq"
                  }
                ]
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Invalidate Translations Based on VPID",
                "det": "Invalidates entries in the TLBs and paging-structure caches based on VPID"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "F0"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "MOVBE",
                "dst": {
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "M",
                  "t": "vqp"
                }
              },
              "grp1": "gen",
              "grp2": "datamov",
              "note": {
                "brief": "Move Data After Swapping Bytes",
                "det": "Reverse byte order in op2 and move to op1"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "F2",
              "sec_opcd": {
                "escape": "yes",
                "#text": "F0"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "CRC32",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "E",
                  "t": "b"
                }
              },
              "instr_ext": "sse42",
              "note": {
                "brief": "Accumulate CRC32 Value",
                "det": "Accumulate CRC32 on r/m8"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "F1"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "MOVBE",
                "dst": {
                  "a": "M",
                  "t": "vqp"
                },
                "src": {
                  "a": "G",
                  "t": "vqp"
                }
              },
              "grp1": "gen",
              "grp2": "datamov",
              "note": {
                "brief": "Move Data After Swapping Bytes",
                "det": "Reverse byte order in op2 and move to op1"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "F2",
              "sec_opcd": {
                "escape": "yes",
                "#text": "F1"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "CRC32",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              "instr_ext": "sse42",
              "note": {
                "brief": "Accumulate CRC32 Value",
                "det": "Accumulate CRC32 on r/m8"
              }
            }
          ]
        },
        {
          "value": "3A",
          "entry": [
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "08"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "ROUNDPS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "ps"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ps"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "conver",
              "note": {
                "brief": "Round Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "09"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "ROUNDPD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "ps"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "pd"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "conver",
              "note": {
                "brief": "Round Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0A"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "ROUNDSS",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "ss"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ss"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "conver",
              "note": {
                "brief": "Round Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0B"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "ROUNDSD",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "sd"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "sd"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "conver",
              "note": {
                "brief": "Round Scalar Double-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0C"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "BLENDPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ps"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Blend Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0D"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "BLENDPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "pd"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Blend Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0E"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PBLENDW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Blend Packed Words"
              }
            },
            {
              "r": "yes",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0F"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PALIGNR",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Align Right"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "0F"
              },
              "proc_start": "12",
              "syntax": {
                "mnem": "PALIGNR",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "ssse3",
              "grp1": "simdint",
              "note": {
                "brief": "Packed Align Right"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "14"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PEXTRB",
                  "dst": {
                    "a": "M",
                    "t": "b"
                  },
                  "src": [
                    {
                      "a": "V",
                      "t": "dq"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mod": "nomem",
                  "mnem": "PEXTRB",
                  "dst": {
                    "a": "R",
                    "t": "dqp"
                  },
                  "src": [
                    {
                      "a": "V",
                      "t": "dq"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Extract Byte"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "15"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "PEXTRW",
                  "dst": {
                    "a": "M",
                    "t": "w"
                  },
                  "src": [
                    {
                      "a": "V",
                      "t": "dq"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mod": "nomem",
                  "mnem": "PEXTRW",
                  "dst": {
                    "a": "R",
                    "t": "dqp"
                  },
                  "src": [
                    {
                      "a": "V",
                      "t": "dq"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Extract Word"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "16"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mnem": "PEXTRD",
                  "dst": {
                    "a": "E",
                    "t": "d"
                  },
                  "src": [
                    {
                      "a": "V",
                      "t": "dq"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mnem": "PEXTRQ",
                  "dst": {
                    "a": "E",
                    "t": "qp"
                  },
                  "src": [
                    {
                      "a": "V",
                      "t": "dq"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Extract Dword/Qword"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "17"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "EXTRACTPS",
                "dst": {
                  "a": "E",
                  "t": "d"
                },
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Extract Packed Single-FP Value"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "20"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "nomem",
                  "mnem": "PINSRB",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": [
                    {
                      "a": "M",
                      "t": "b"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mod": "mem",
                  "mnem": "PINSRB",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": [
                    {
                      "a": "R",
                      "t": "dqp"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Insert Byte"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "21"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mod": "mem",
                  "mnem": "INSERTPS",
                  "dst": {
                    "a": "V",
                    "t": "ps"
                  },
                  "src": [
                    {
                      "a": "U",
                      "t": "ps"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mod": "nomem",
                  "mnem": "INSERTPS",
                  "dst": {
                    "a": "V",
                    "t": "ps"
                  },
                  "src": [
                    {
                      "a": "M",
                      "t": "d"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Insert Packed Single-FP Value"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "22"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": [
                {
                  "mnem": "PINSRD",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": [
                    {
                      "a": "E",
                      "t": "d"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mnem": "PINSRQ",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": [
                    {
                      "a": "E",
                      "t": "qp"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Insert Dword/Qword"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "40"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "DPPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Dot Product of Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "41"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "DPPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse41",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Dot Product of Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "42"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "MPSADBW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse41",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Compute Multiple Packed Sums of Absolute Difference"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "60"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PCMPESTRM",
                "dst": {
                  "nr": "0",
                  "group": "xmm",
                  "displayed": "no",
                  "#text": "XMM0"
                },
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "dqp",
                    "displayed": "no",
                    "#text": "rAX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "dqp",
                    "displayed": "no",
                    "#text": "rDX"
                  }
                ]
              },
              "instr_ext": "sse42",
              "grp1": "strtxt",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "f_vals": "ap",
              "note": {
                "brief": "Packed Compare Explicit Length Strings, Return Mask",
                "det": "Perform a packed comparison of string data with\n\t  explicit lengths, generating a mask, and storing the result\n\t  in XMM0"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "61"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PCMPESTRI",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rCX"
                },
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "dqp",
                    "displayed": "no",
                    "#text": "rAX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "dqp",
                    "displayed": "no",
                    "#text": "rDX"
                  }
                ]
              },
              "instr_ext": "sse42",
              "grp1": "strtxt",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "f_vals": "ap",
              "note": {
                "brief": "Packed Compare Explicit Length Strings, Return Index",
                "det": "Perform a packed comparison of string data with\n\t  explicit lengths, generating an index, and storing the\n\t  result in rCX"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "62"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PCMPISTRM",
                "dst": {
                  "nr": "0",
                  "group": "xmm",
                  "displayed": "no",
                  "#text": "XMM0"
                },
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse42",
              "grp1": "strtxt",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "f_vals": "ap",
              "note": {
                "brief": "Packed Compare Implicit Length Strings, Return Mask",
                "det": "Perform a packed comparison of string data with\n\t  implicit lengths, generating a mask, and storing the result\n\t  in XMM0"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_SSE4_amd",
              "pref": "66",
              "sec_opcd": {
                "escape": "yes",
                "#text": "63"
              },
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "PCMPISTRI",
                "dst": {
                  "nr": "1",
                  "group": "gen",
                  "type": "dqp",
                  "displayed": "no",
                  "#text": "rCX"
                },
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse42",
              "grp1": "strtxt",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "f_vals": "ap",
              "note": {
                "brief": "Packed Compare Implicit Length Strings, Return Index",
                "det": "Perform a packed comparison of string data with\n\t  implicit lengths, generating an index, and storing the\n\t  result in rCX"
              }
            }
          ]
        },
        {
          "value": "40",
          "entry": {
            "tttn": "0000",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": {
              "mnem": "CMOVO",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "o",
            "note": {
              "brief": "Conditional Move - overflow (OF=1)"
            }
          }
        },
        {
          "value": "41",
          "entry": {
            "tttn": "0001",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": {
              "mnem": "CMOVNO",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "o",
            "note": {
              "brief": "Conditional Move - not overflow (OF=0)"
            }
          }
        },
        {
          "value": "42",
          "entry": {
            "tttn": "0010",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVB",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVNAE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVC",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "c",
            "note": {
              "brief": "Conditional Move - below/not above or equal/carry (CF=1)"
            }
          }
        },
        {
          "value": "43",
          "entry": {
            "tttn": "0011",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVNB",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVAE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVNC",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "c",
            "note": {
              "brief": "Conditional Move - not below/above or equal/not carry (CF=0)"
            }
          }
        },
        {
          "value": "44",
          "entry": {
            "tttn": "0100",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVZ",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "z",
            "note": {
              "brief": "Conditional Move - zero/equal (ZF=0)"
            }
          }
        },
        {
          "value": "45",
          "entry": {
            "tttn": "0101",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVNZ",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVNE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "z",
            "note": {
              "brief": "Conditional Move - not zero/not equal (ZF=1)"
            }
          }
        },
        {
          "value": "46",
          "entry": {
            "tttn": "0110",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVBE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVNA",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "zc",
            "note": {
              "brief": "Conditional Move - below or equal/not above (CF=1 AND ZF=1)"
            }
          }
        },
        {
          "value": "47",
          "entry": {
            "tttn": "0111",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVNBE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVA",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "zc",
            "note": {
              "brief": "Conditional Move - not below or equal/above (CF=0 AND ZF=0)"
            }
          }
        },
        {
          "value": "48",
          "entry": {
            "tttn": "1000",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": {
              "mnem": "CMOVS",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "s",
            "note": {
              "brief": "Conditional Move - sign (SF=1)"
            }
          }
        },
        {
          "value": "49",
          "entry": {
            "tttn": "1001",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": {
              "mnem": "CMOVNS",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "s",
            "note": {
              "brief": "Conditional Move - not sign (SF=0)"
            }
          }
        },
        {
          "value": "4A",
          "entry": {
            "tttn": "1010",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVP",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVPE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "p",
            "note": {
              "brief": "Conditional Move - parity/parity even (PF=1)"
            }
          }
        },
        {
          "value": "4B",
          "entry": {
            "tttn": "1011",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVNP",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVPO",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "p",
            "note": {
              "brief": "Conditional Move - not parity/parity odd"
            }
          }
        },
        {
          "value": "4C",
          "entry": {
            "tttn": "1100",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVL",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVNGE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "os",
            "note": {
              "brief": "Conditional Move - less/not greater (SF!=OF)"
            }
          }
        },
        {
          "value": "4D",
          "entry": {
            "tttn": "1101",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVNL",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVGE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "os",
            "note": {
              "brief": "Conditional Move - not less/greater or equal (SF=OF)"
            }
          }
        },
        {
          "value": "4E",
          "entry": {
            "tttn": "1110",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVLE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVNG",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "osz",
            "note": {
              "brief": "Conditional Move - less or equal/not greater ((ZF=1) OR (SF!=OF))"
            }
          }
        },
        {
          "value": "4F",
          "entry": {
            "tttn": "1111",
            "r": "yes",
            "doc64_ref": "gen_note_CMOVcc_0F40-0F4F",
            "proc_start": "07",
            "syntax": [
              {
                "mnem": "CMOVNLE",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              {
                "mnem": "CMOVG",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "osz",
            "note": {
              "brief": "Conditional Move - not less nor equal/greater ((ZF=0) AND (SF=OF))"
            }
          }
        },
        {
          "value": "50",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVMSKPS",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "U",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "datamov",
              "note": {
                "brief": "Extract Packed Single-FP Sign Mask"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVMSKPD",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "U",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "datamov",
              "note": {
                "brief": "Extract Packed Double-FP Sign Mask"
              }
            }
          ]
        },
        {
          "value": "51",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "SQRTPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Compute Square Roots of Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "SQRTSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Compute Square Root of Scalar Single-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "SQRTPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Compute Square Roots of Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "SQRTSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Compute Square Root of Scalar Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "52",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "RSQRTPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Compute Recipr. of Square Roots of Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "RSQRTSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Compute Recipr. of Square Root of Scalar Single-FP Value"
              }
            }
          ]
        },
        {
          "value": "53",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "RCPPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Compute Reciprocals of Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "RCPSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Compute Reciprocal of Scalar Single-FP Values"
              }
            }
          ]
        },
        {
          "value": "54",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "ANDPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical AND of Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "ANDPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical AND of Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "55",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "ANDNPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical AND NOT of Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "ANDNPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical AND NOT of Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "56",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "ORPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical OR of Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "ORPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical OR of Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "57",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "XORPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical XOR for Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "XORPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical XOR for Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "58",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "ADDPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "ADDSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Add Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "ADDPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "ADDSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Add Scalar Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "59",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MULPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "MULSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Scalar Single-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MULPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "MULSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Scalar Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "5A",
          "entry": [
            {
              "r": "yes",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTPS2PD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert Packed Single-FP Values to",
                  "Double-FP Values"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTPD2PS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert Packed Double-FP Values to",
                  "Single-FP Values"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTSS2SD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": "Convert Scalar Single-FP Value to Scalar Double-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTSD2SS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": "Convert Scalar Double-FP Value to Scalar Single-FP Value"
              }
            }
          ]
        },
        {
          "value": "5B",
          "entry": [
            {
              "r": "yes",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTDQ2PS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksp",
              "note": {
                "brief": [
                  "Convert Packed DW Integers to",
                  "Single-FP Values"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTPS2DQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksp",
              "note": {
                "brief": [
                  "Convert Packed Single-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTTPS2DQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksp",
              "note": {
                "brief": [
                  "Convert with Trunc. Packed Single-FP Values to",
                  "DW Integers"
                ]
              }
            }
          ]
        },
        {
          "value": "5C",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "SUBPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "SUBSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "SUBPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "SUBSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Scalar Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "5D",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MINPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Return Minimum Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "MINSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Return Minimum Scalar Single-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MINPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Return Minimum Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "MINSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Return Minimum Scalar Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "5E",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "DIVPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Divide Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "DIVSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Divide Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "DIVPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Divide Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "DIVSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Divide Scalar Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "5F",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MAXPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Return Maximum Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "MAXSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": {
                  "a": "W",
                  "t": "ss"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Return Maximum Scalar Single-FP Value"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MAXPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Return Maximum Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "MAXSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": {
                  "a": "W",
                  "t": "sd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "arith",
              "note": {
                "brief": "Return Maximum Scalar Double-FP Value"
              }
            }
          ]
        },
        {
          "value": "60",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PUNPCKLBW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "unpack",
              "note": {
                "brief": "Unpack Low Data"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PUNPCKLBW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack Low Data"
              }
            }
          ]
        },
        {
          "value": "61",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PUNPCKLWD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "unpack",
              "note": {
                "brief": "Unpack Low Data"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PUNPCKLWD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack Low Data"
              }
            }
          ]
        },
        {
          "value": "62",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PUNPCKLDQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "unpack",
              "note": {
                "brief": "Unpack Low Data"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PUNPCKLDQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack Low Data"
              }
            }
          ]
        },
        {
          "value": "63",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PACKSSWB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "conver",
              "note": {
                "brief": "Pack with Signed Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PACKSSWB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Pack with Signed Saturation"
              }
            }
          ]
        },
        {
          "value": "64",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PCMPGTB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "compar",
              "note": {
                "brief": "Compare Packed Signed Integers for Greater Than"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PCMPGTB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Signed Integers for Greater Than"
              }
            }
          ]
        },
        {
          "value": "65",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PCMPGTW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "compar",
              "note": {
                "brief": "Compare Packed Signed Integers for Greater Than"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PCMPGTW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Signed Integers for Greater Than"
              }
            }
          ]
        },
        {
          "value": "66",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PCMPGTD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "compar",
              "note": {
                "brief": "Compare Packed Signed Integers for Greater Than"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PCMPGTD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Signed Integers for Greater Than"
              }
            }
          ]
        },
        {
          "value": "67",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PACKUSWB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "conver",
              "note": {
                "brief": "Pack with Unsigned Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PACKUSWB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Pack with Unsigned Saturation"
              }
            }
          ]
        },
        {
          "value": "68",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PUNPCKHBW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "unpack",
              "note": {
                "brief": "Unpack High Data"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PUNPCKHBW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack High Data"
              }
            }
          ]
        },
        {
          "value": "69",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PUNPCKHWD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "unpack",
              "note": {
                "brief": "Unpack High Data"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PUNPCKHWD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack High Data"
              }
            }
          ]
        },
        {
          "value": "6A",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PUNPCKHDQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "unpack",
              "note": {
                "brief": "Unpack High Data"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PUNPCKHDQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Unpack High Data"
              }
            }
          ]
        },
        {
          "value": "6B",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PACKSSDW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "conver",
              "note": {
                "brief": "Pack with Signed Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PACKSSDW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "conver",
              "note": {
                "brief": "Pack with Signed Saturation"
              }
            }
          ]
        },
        {
          "value": "6C",
          "entry": {
            "r": "yes",
            "pref": "66",
            "proc_start": "10",
            "syntax": {
              "mnem": "PUNPCKLQDQ",
              "dst": {
                "a": "V",
                "t": "dq"
              },
              "src": {
                "a": "W",
                "t": "dq"
              }
            },
            "instr_ext": "sse2",
            "grp1": "simdint",
            "grp2": "shunpck",
            "note": {
              "brief": "Unpack Low Data"
            }
          }
        },
        {
          "value": "6D",
          "entry": {
            "r": "yes",
            "pref": "66",
            "proc_start": "10",
            "syntax": {
              "mnem": "PUNPCKHQDQ",
              "dst": {
                "a": "V",
                "t": "dq"
              },
              "src": {
                "a": "W",
                "t": "dq"
              }
            },
            "instr_ext": "sse2",
            "grp1": "simdint",
            "grp2": "shunpck",
            "note": {
              "brief": "Unpack High Data"
            }
          }
        },
        {
          "value": "6E",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "MOVD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "E",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "datamov",
              "note": {
                "brief": "Move Doubleword"
              }
            },
            {
              "r": "yes",
              "doc64_ref": "gen_note_MOVQ_0F6E_660F6E_0F7E_660F7E",
              "mode": "e",
              "proc_start": "10",
              "syntax": [
                {
                  "mnem": "MOVD",
                  "dst": {
                    "a": "P",
                    "t": "q"
                  },
                  "src": {
                    "a": "E",
                    "t": "d"
                  }
                },
                {
                  "mnem": "MOVQ",
                  "dst": {
                    "depend": "no",
                    "a": "P",
                    "t": "q"
                  },
                  "src": {
                    "a": "E",
                    "t": "qp"
                  }
                }
              ],
              "instr_ext": "mmx",
              "grp1": "datamov",
              "note": {
                "brief": "Move Doubleword/Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "E",
                  "t": "d"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Doubleword"
              }
            },
            {
              "r": "yes",
              "doc64_ref": "gen_note_MOVQ_0F6E_660F6E_0F7E_660F7E",
              "mode": "e",
              "pref": "66",
              "proc_start": "10",
              "syntax": [
                {
                  "mnem": "MOVD",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "E",
                    "t": "d"
                  }
                },
                {
                  "mnem": "MOVQ",
                  "dst": {
                    "depend": "no",
                    "a": "V",
                    "t": "dq"
                  },
                  "src": {
                    "a": "E",
                    "t": "qp"
                  }
                }
              ],
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Doubleword/Quadword"
              }
            }
          ]
        },
        {
          "value": "6F",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "MOVQ",
                "dst": {
                  "depend": "no",
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "datamov",
              "note": {
                "brief": "Move Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVDQA",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Aligned Double Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVDQU",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Unaligned Double Quadword"
              }
            }
          ]
        },
        {
          "value": "70",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PSHUFW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": [
                  {
                    "a": "Q",
                    "t": "q"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Shuffle Packed Words"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSHUFLW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Shuffle Packed Low Words"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSHUFHW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Shuffle Packed High Words"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSHUFD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shunpck",
              "note": {
                "brief": "Shuffle Packed Doublewords"
              }
            }
          ]
        },
        {
          "value": "71",
          "entry": [
            {
              "opcd_ext": "2",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRLW",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "opcd_ext": "2",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLW",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "opcd_ext": "4",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRAW",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            },
            {
              "opcd_ext": "4",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRAW",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            },
            {
              "opcd_ext": "6",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSLLW",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "opcd_ext": "6",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLW",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            }
          ]
        },
        {
          "value": "72",
          "entry": [
            {
              "opcd_ext": "2",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRLD",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Double Quadword Right Logical"
              }
            },
            {
              "opcd_ext": "2",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLD",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Double Quadword Right Logical"
              }
            },
            {
              "opcd_ext": "4",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRAD",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            },
            {
              "opcd_ext": "4",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRAD",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            },
            {
              "opcd_ext": "6",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSLLD",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "opcd_ext": "6",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLD",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            }
          ]
        },
        {
          "value": "73",
          "entry": [
            {
              "opcd_ext": "2",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRLQ",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "opcd_ext": "2",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLQ",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "opcd_ext": "3",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLDQ",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Double Quadword Right Logical"
              }
            },
            {
              "opcd_ext": "6",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSLLQ",
                "dst": {
                  "a": "N",
                  "t": "q"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "opcd_ext": "6",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLQ",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "opcd_ext": "7",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLDQ",
                "dst": {
                  "a": "U",
                  "t": "dq"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Double Quadword Left Logical"
              }
            }
          ]
        },
        {
          "value": "74",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PCMPEQB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "compar",
              "note": {
                "brief": "Compare Packed Data for Equal"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PCMPEQB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Data for Equal"
              }
            }
          ]
        },
        {
          "value": "75",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PCMPEQW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "compar",
              "note": {
                "brief": "Compare Packed Data for Equal"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PCMPEQW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Data for Equal"
              }
            }
          ]
        },
        {
          "value": "76",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PCMPEQD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "compar",
              "note": {
                "brief": "Compare Packed Data for Equal"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PCMPEQD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Data for Equal"
              }
            }
          ]
        },
        {
          "value": "77",
          "entry": {
            "proc_start": "06",
            "syntax": {
              "mnem": "EMMS"
            },
            "instr_ext": "mmx",
            "grp1": "x87fpu",
            "grp2": "control",
            "note": {
              "brief": "Empty MMX Technology State"
            }
          }
        },
        {
          "value": "78",
          "entry": [
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMREAD",
                "dst": {
                  "a": "E",
                  "t": "d"
                },
                "src": {
                  "a": "G",
                  "t": "d"
                }
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Read Field from Virtual-Machine Control Structure",
                "det": "Reads a specified VMCS field"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "e",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMREAD",
                "dst": {
                  "a": "E",
                  "t": "q"
                },
                "src": {
                  "a": "G",
                  "t": "q"
                }
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Read Field from Virtual-Machine Control Structure",
                "det": "Reads a specified VMCS field"
              }
            }
          ]
        },
        {
          "value": "79",
          "entry": [
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMWRITE",
                "src": [
                  {
                    "a": "G",
                    "t": "d"
                  },
                  {
                    "a": "E",
                    "t": "d"
                  }
                ]
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Write Field to Virtual-Machine Control Structure",
                "det": "Writes a specified VMCS field"
              }
            },
            {
              "r": "yes",
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "e",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMWRITE",
                "src": [
                  {
                    "a": "G",
                    "t": "q"
                  },
                  {
                    "a": "E",
                    "t": "q"
                  }
                ]
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Write Field to Virtual-Machine Control Structure",
                "det": "Writes a specified VMCS field"
              }
            }
          ]
        },
        {
          "value": "7C",
          "entry": [
            {
              "r": "yes",
              "pref": "66",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "HADDPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Packed Double-FP Horizontal Add"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "HADDPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Packed Single-FP Horizontal Add"
              }
            }
          ]
        },
        {
          "value": "7D",
          "entry": [
            {
              "r": "yes",
              "pref": "66",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "HSUBPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Packed Double-FP Horizontal Subtract"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "HSUBPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Packed Single-FP Horizontal Subtract"
              }
            }
          ]
        },
        {
          "value": "7E",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "MOVD",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "d"
                },
                "src": {
                  "a": "P",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "datamov",
              "note": {
                "brief": "Move Doubleword"
              }
            },
            {
              "r": "yes",
              "doc64_ref": "gen_note_MOVQ_0F6E_660F6E_0F7E_660F7E",
              "mode": "e",
              "proc_start": "10",
              "syntax": [
                {
                  "mnem": "MOVD",
                  "dst": {
                    "depend": "no",
                    "a": "E",
                    "t": "d"
                  },
                  "src": {
                    "a": "P",
                    "t": "q"
                  }
                },
                {
                  "mnem": "MOVQ",
                  "dst": {
                    "depend": "no",
                    "a": "E",
                    "t": "qp"
                  },
                  "src": {
                    "a": "P",
                    "t": "q"
                  }
                }
              ],
              "instr_ext": "mmx",
              "grp1": "datamov",
              "note": {
                "brief": "Move Doubleword/Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVD",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "d"
                },
                "src": {
                  "a": "V",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Doubleword"
              }
            },
            {
              "r": "yes",
              "doc64_ref": "gen_note_MOVQ_0F6E_660F6E_0F7E_660F7E",
              "mode": "e",
              "pref": "66",
              "proc_start": "10",
              "syntax": [
                {
                  "mnem": "MOVD",
                  "dst": {
                    "depend": "no",
                    "a": "E",
                    "t": "d"
                  },
                  "src": {
                    "a": "V",
                    "t": "dq"
                  }
                },
                {
                  "mnem": "MOVQ",
                  "dst": {
                    "depend": "no",
                    "a": "E",
                    "t": "qp"
                  },
                  "src": {
                    "a": "E",
                    "t": "dq"
                  }
                }
              ],
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Doubleword/Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVQ",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "q"
                },
                "src": {
                  "a": "W",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Quadword"
              }
            }
          ]
        },
        {
          "value": "7F",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "MOVQ",
                "dst": {
                  "depend": "no",
                  "a": "Q",
                  "t": "q"
                },
                "src": {
                  "a": "P",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "datamov",
              "note": {
                "brief": "Move Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVDQA",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "dq"
                },
                "src": {
                  "a": "V",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Aligned Double Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVDQU",
                "dst": {
                  "depend": "no",
                  "a": "W",
                  "t": "dq"
                },
                "src": {
                  "a": "V",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Unaligned Double Quadword"
              }
            }
          ]
        },
        {
          "value": "80",
          "entry": {
            "tttn": "0000",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": {
              "mnem": "JO",
              "src": {
                "a": "J",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "o",
            "note": {
              "brief": "Jump short if overflow (OF=1)"
            }
          }
        },
        {
          "value": "81",
          "entry": {
            "tttn": "0001",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": {
              "mnem": "JNO",
              "src": {
                "a": "J",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "o",
            "note": {
              "brief": "Jump short if not overflow (OF=0)"
            }
          }
        },
        {
          "value": "82",
          "entry": {
            "tttn": "0010",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JB",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JNAE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JC",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "c",
            "note": {
              "brief": "Jump short if below/not above or equal/carry (CF=1)"
            }
          }
        },
        {
          "value": "83",
          "entry": {
            "tttn": "0011",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JNB",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JAE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JNC",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "c",
            "note": {
              "brief": "Jump short if not below/above or equal/not carry (CF=0)"
            }
          }
        },
        {
          "value": "84",
          "entry": {
            "tttn": "0100",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JZ",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "z",
            "note": {
              "brief": "Jump short if zero/equal (ZF=0)"
            }
          }
        },
        {
          "value": "85",
          "entry": {
            "tttn": "0101",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JNZ",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JNE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "z",
            "note": {
              "brief": "Jump short if not zero/not equal (ZF=1)"
            }
          }
        },
        {
          "value": "86",
          "entry": {
            "tttn": "0110",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JBE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JNA",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "zc",
            "note": {
              "brief": "Jump short if below or equal/not above (CF=1 AND ZF=1)"
            }
          }
        },
        {
          "value": "87",
          "entry": {
            "tttn": "0111",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JNBE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JA",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "zc",
            "note": {
              "brief": "Jump short if not below or equal/above (CF=0 AND ZF=0)"
            }
          }
        },
        {
          "value": "88",
          "entry": {
            "tttn": "1000",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": {
              "mnem": "JS",
              "src": {
                "a": "J",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "s",
            "note": {
              "brief": "Jump short if sign (SF=1)"
            }
          }
        },
        {
          "value": "89",
          "entry": {
            "tttn": "1001",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": {
              "mnem": "JNS",
              "src": {
                "a": "J",
                "t": "vds"
              }
            },
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "s",
            "note": {
              "brief": "Jump short if not sign (SF=0)"
            }
          }
        },
        {
          "value": "8A",
          "entry": {
            "tttn": "1010",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JP",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JPE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "p",
            "note": {
              "brief": "Jump short if parity/parity even (PF=1)"
            }
          }
        },
        {
          "value": "8B",
          "entry": {
            "tttn": "1011",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JNP",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JPO",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "p",
            "note": {
              "brief": "Jump short if not parity/parity odd"
            }
          }
        },
        {
          "value": "8C",
          "entry": {
            "tttn": "1100",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JL",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JNGE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "os",
            "note": {
              "brief": "Jump short if less/not greater (SF!=OF)"
            }
          }
        },
        {
          "value": "8D",
          "entry": {
            "tttn": "1101",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JNL",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JGE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "os",
            "note": {
              "brief": "Jump short if not less/greater or equal (SF=OF)"
            }
          }
        },
        {
          "value": "8E",
          "entry": {
            "tttn": "1110",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JLE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JNG",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "osz",
            "note": {
              "brief": "Jump short if less or equal/not greater ((ZF=1) OR (SF!=OF))"
            }
          }
        },
        {
          "value": "8F",
          "entry": {
            "tttn": "1111",
            "doc64_ref": "gen_note_short_near_jmp",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "JNLE",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              },
              {
                "mnem": "JG",
                "src": {
                  "a": "J",
                  "t": "vds"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "branch",
            "grp3": "cond",
            "test_f": "osz",
            "note": {
              "brief": "Jump short if not less nor equal/greater ((ZF=0) AND (SF=OF))"
            }
          }
        },
        {
          "value": "90",
          "entry": {
            "tttn": "0000",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": {
              "mnem": "SETO",
              "dst": {
                "depend": "no",
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "o",
            "note": {
              "brief": "Set Byte on Condition - overflow (OF=1)"
            }
          }
        },
        {
          "value": "91",
          "entry": {
            "tttn": "0001",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": {
              "mnem": "SETNO",
              "dst": {
                "depend": "no",
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "o",
            "note": {
              "brief": "Set Byte on Condition - not overflow (OF=0)"
            }
          }
        },
        {
          "value": "92",
          "entry": {
            "tttn": "0010",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETB",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETNAE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETC",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "c",
            "note": {
              "brief": "Set Byte on Condition - below/not above or equal/carry (CF=1)"
            }
          }
        },
        {
          "value": "93",
          "entry": {
            "tttn": "0011",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETNB",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETAE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETNC",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "c",
            "note": {
              "brief": "Set Byte on Condition - not below/above or equal/not carry (CF=0)"
            }
          }
        },
        {
          "value": "94",
          "entry": {
            "tttn": "0100",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETZ",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "z",
            "note": {
              "brief": "Set Byte on Condition - zero/equal (ZF=0)"
            }
          }
        },
        {
          "value": "95",
          "entry": {
            "tttn": "0101",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETNZ",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETNE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "z",
            "note": {
              "brief": "Set Byte on Condition - not zero/not equal (ZF=1)"
            }
          }
        },
        {
          "value": "96",
          "entry": {
            "tttn": "0110",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETBE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETNA",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "zc",
            "note": {
              "brief": "Set Byte on Condition - below or equal/not above (CF=1 AND ZF=1)"
            }
          }
        },
        {
          "value": "97",
          "entry": {
            "tttn": "0111",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETNBE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETA",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "zc",
            "note": {
              "brief": "Set Byte on Condition - not below or equal/above (CF=0 AND ZF=0)"
            }
          }
        },
        {
          "value": "98",
          "entry": {
            "tttn": "1000",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": {
              "mnem": "SETS",
              "dst": {
                "depend": "no",
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "s",
            "note": {
              "brief": "Set Byte on Condition - sign (SF=1)"
            }
          }
        },
        {
          "value": "99",
          "entry": {
            "tttn": "1001",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": {
              "mnem": "SETNS",
              "dst": {
                "depend": "no",
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "s",
            "note": {
              "brief": "Set Byte on Condition - not sign (SF=0)"
            }
          }
        },
        {
          "value": "9A",
          "entry": {
            "tttn": "1010",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETP",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETPE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "p",
            "note": {
              "brief": "Set Byte on Condition - parity/parity even (PF=1)"
            }
          }
        },
        {
          "value": "9B",
          "entry": {
            "tttn": "1011",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETNP",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETPO",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "p",
            "note": {
              "brief": "Set Byte on Condition - not parity/parity odd"
            }
          }
        },
        {
          "value": "9C",
          "entry": {
            "tttn": "1100",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETL",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETNGE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "os",
            "note": {
              "brief": "Set Byte on Condition - less/not greater (SF!=OF)"
            }
          }
        },
        {
          "value": "9D",
          "entry": {
            "tttn": "1101",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETNL",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETGE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "os",
            "note": {
              "brief": "Set Byte on Condition - not less/greater or equal (SF=OF)"
            }
          }
        },
        {
          "value": "9E",
          "entry": {
            "tttn": "1110",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETLE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETNG",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "osz",
            "note": {
              "brief": "Set Byte on Condition - less or equal/not greater ((ZF=1) OR (SF!=OF))"
            }
          }
        },
        {
          "value": "9F",
          "entry": {
            "tttn": "1111",
            "doc_ref": "gen_note_SETcc_0F90-0F9F",
            "opcd_ext": "0",
            "proc_start": "03",
            "syntax": [
              {
                "mnem": "SETNLE",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              },
              {
                "mnem": "SETG",
                "dst": {
                  "depend": "no",
                  "a": "E",
                  "t": "b"
                }
              }
            ],
            "grp1": "gen",
            "grp2": "datamov",
            "test_f": "osz",
            "note": {
              "brief": "Set Byte on Condition - not less nor equal/greater ((ZF=0) AND (SF=OF))"
            }
          }
        },
        {
          "value": "A0",
          "entry": {
            "proc_start": "03",
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "nr": "4",
                "group": "seg",
                "type": "w",
                "address": "S33",
                "#text": "FS"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          }
        },
        {
          "value": "A1",
          "entry": {
            "proc_start": "03",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "nr": "4",
                "group": "seg",
                "type": "w",
                "address": "S33",
                "depend": "no",
                "#text": "FS"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          }
        },
        {
          "value": "A2",
          "entry": {
            "attr": "serial",
            "proc_start": {
              "lat_step": "yes",
              "#text": "04"
            },
            "syntax": {
              "mnem": "CPUID",
              "dst": [
                {
                  "nr": "8B",
                  "group": "msr",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "IA32_BIOS_SIGN_ID"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "d",
                  "displayed": "no",
                  "#text": "EAX"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "ECX"
                },
                {
                  "nr": "2",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EDX"
                },
                {
                  "nr": "3",
                  "group": "gen",
                  "type": "d",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "EBX"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "CPU Identification"
            }
          }
        },
        {
          "value": "A3",
          "entry": {
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "BT",
              "src": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "a": "G",
                  "t": "vqp"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "bit",
            "modif_f": "oszapc",
            "def_f": "c",
            "undef_f": "oszap",
            "note": {
              "brief": "Bit Test"
            }
          }
        },
        {
          "value": "A4",
          "entry": {
            "direction": "0",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "SHLD",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": [
                {
                  "a": "G",
                  "t": "vqp"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Double Precision Shift Left"
            }
          }
        },
        {
          "value": "A5",
          "entry": {
            "direction": "0",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "SHLD",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": [
                {
                  "a": "G",
                  "t": "vqp"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Double Precision Shift Left"
            }
          }
        },
        {
          "value": "A8",
          "entry": {
            "proc_start": "03",
            "syntax": {
              "mnem": "PUSH",
              "dst": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              },
              "src": {
                "nr": "5",
                "group": "seg",
                "type": "w",
                "address": "S33",
                "#text": "GS"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Push Word, Doubleword or Quadword Onto the Stack"
            }
          }
        },
        {
          "value": "A9",
          "entry": {
            "proc_start": "03",
            "syntax": {
              "mnem": "POP",
              "dst": {
                "nr": "5",
                "group": "seg",
                "type": "w",
                "address": "S33",
                "depend": "no",
                "#text": "GS"
              },
              "src": {
                "address": "SC",
                "displayed": "no",
                "#text": "SS:[rSP]"
              }
            },
            "grp1": "gen",
            "grp2": [
              "stack",
              "segreg"
            ],
            "note": {
              "brief": "Pop a Value from the Stack"
            }
          }
        },
        {
          "value": "AA",
          "entry": {
            "mode": "s",
            "particular": "yes",
            "proc_start": {
              "lat_step": "yes",
              "#text": "03"
            },
            "syntax": {
              "mnem": "RSM",
              "dst": {
                "type": "w",
                "address": "F",
                "depend": "no",
                "displayed": "no",
                "#text": "Flags"
              }
            },
            "grp1": "system",
            "grp2": "branch",
            "note": {
              "brief": "Resume from System Management Mode"
            }
          }
        },
        {
          "value": "AB",
          "entry": {
            "r": "yes",
            "lock": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "BTS",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "G",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "bit",
            "modif_f": "oszapc",
            "def_f": "c",
            "undef_f": "oszap",
            "note": {
              "brief": "Bit Test and Set"
            }
          }
        },
        {
          "value": "AC",
          "entry": {
            "direction": "0",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "SHRD",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": [
                {
                  "a": "G",
                  "t": "vqp"
                },
                {
                  "a": "I",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Double Precision Shift Right"
            }
          }
        },
        {
          "value": "AD",
          "entry": {
            "direction": "0",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "SHRD",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": [
                {
                  "a": "G",
                  "t": "vqp"
                },
                {
                  "nr": "1",
                  "group": "gen",
                  "type": "b",
                  "#text": "CL"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "shftrot",
            "modif_f": "oszapc",
            "def_f": "oszpc",
            "undef_f": "oac",
            "note": {
              "brief": "Double Precision Shift Right"
            }
          }
        },
        {
          "value": "AE",
          "entry": [
            {
              "opcd_ext": "0",
              "proc_start": {
                "lat_step": "yes",
                "#text": "08"
              },
              "syntax": {
                "mnem": "FXSAVE",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "stx"
                },
                "src": [
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM7"
                  }
                ]
              },
              "grp1": "sm",
              "note": {
                "brief": "Save x87 FPU, MMX, XMM, and MXCSR State"
              }
            },
            {
              "mode": "e",
              "opcd_ext": "0",
              "proc_start": "10",
              "syntax": {
                "mnem": "FXSAVE",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "stx"
                },
                "src": [
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM7"
                  },
                  {
                    "nr": "8",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM8"
                  },
                  {
                    "nr": "9",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM9"
                  },
                  {
                    "nr": "10",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM10"
                  },
                  {
                    "nr": "11",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM11"
                  },
                  {
                    "nr": "12",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM12"
                  },
                  {
                    "nr": "13",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM13"
                  },
                  {
                    "nr": "14",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM14"
                  },
                  {
                    "nr": "15",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM15"
                  }
                ]
              },
              "grp1": "sm",
              "note": {
                "brief": "Save x87 FPU, MMX, XMM, and MXCSR State"
              }
            },
            {
              "opcd_ext": "1",
              "proc_start": {
                "lat_step": "yes",
                "#text": "08"
              },
              "syntax": {
                "mnem": "FXRSTOR",
                "dst": [
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM7"
                  }
                ],
                "src": {
                  "a": "M",
                  "t": "stx"
                }
              },
              "grp1": "sm",
              "note": {
                "brief": "Restore x87 FPU, MMX, XMM, and MXCSR State"
              }
            },
            {
              "mode": "e",
              "opcd_ext": "1",
              "proc_start": "10",
              "syntax": {
                "mnem": "FXRSTOR",
                "dst": [
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM7"
                  },
                  {
                    "nr": "8",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM8"
                  },
                  {
                    "nr": "9",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM9"
                  },
                  {
                    "nr": "10",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM10"
                  },
                  {
                    "nr": "11",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM11"
                  },
                  {
                    "nr": "12",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM12"
                  },
                  {
                    "nr": "13",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM13"
                  },
                  {
                    "nr": "14",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM14"
                  },
                  {
                    "nr": "15",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM15"
                  }
                ],
                "src": {
                  "a": "M",
                  "t": "stx"
                }
              },
              "grp1": "sm",
              "note": {
                "brief": "Restore x87 FPU, MMX, XMM, and MXCSR State"
              }
            },
            {
              "opcd_ext": "2",
              "proc_start": "09",
              "syntax": {
                "mnem": "LDMXCSR",
                "src": {
                  "a": "M",
                  "t": "d"
                }
              },
              "instr_ext": "sse1",
              "grp1": "mxcsrsm",
              "note": {
                "brief": "Load MXCSR Register"
              }
            },
            {
              "opcd_ext": "3",
              "proc_start": "09",
              "syntax": {
                "mnem": "STMXCSR",
                "dst": {
                  "a": "M",
                  "t": "d"
                }
              },
              "instr_ext": "sse1",
              "grp1": "mxcsrsm",
              "note": {
                "brief": "Store MXCSR Register State"
              }
            },
            {
              "opcd_ext": "4",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "XSAVE",
                "dst": {
                  "a": "M"
                },
                "src": [
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  },
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM7"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Save Processor Extended States",
                "det": "Save processor extended states to memory. The states are specified by EDX:EAX"
              }
            },
            {
              "mode": "e",
              "opcd_ext": "4",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "XSAVE",
                "dst": {
                  "a": "M"
                },
                "src": [
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  },
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM7"
                  },
                  {
                    "nr": "8",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM8"
                  },
                  {
                    "nr": "9",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM9"
                  },
                  {
                    "nr": "10",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM10"
                  },
                  {
                    "nr": "11",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM11"
                  },
                  {
                    "nr": "12",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM12"
                  },
                  {
                    "nr": "13",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM13"
                  },
                  {
                    "nr": "14",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM14"
                  },
                  {
                    "nr": "15",
                    "group": "xmm",
                    "displayed": "no",
                    "#text": "XMM15"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Save Processor Extended States",
                "det": "Save processor extended states to memory. The states are specified by EDX:EAX"
              }
            },
            {
              "mod": "nomem",
              "opcd_ext": "5",
              "proc_start": "10",
              "syntax": {
                "mnem": "LFENCE"
              },
              "instr_ext": "sse2",
              "grp1": "order",
              "note": {
                "brief": "Load Fence"
              }
            },
            {
              "mod": "mem",
              "opcd_ext": "5",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "XRSTOR",
                "dst": [
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM7"
                  }
                ],
                "src": [
                  {
                    "a": "M"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Restore Processor Extended States",
                "det": "Restore processor extended states from memory. The states are specified by EDX:EAX"
              }
            },
            {
              "mod": "mem",
              "mode": "e",
              "opcd_ext": "5",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "XRSTOR",
                "dst": [
                  {
                    "nr": "0",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST"
                  },
                  {
                    "nr": "1",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST1"
                  },
                  {
                    "nr": "2",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST2"
                  },
                  {
                    "nr": "3",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST3"
                  },
                  {
                    "nr": "4",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST4"
                  },
                  {
                    "nr": "5",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST5"
                  },
                  {
                    "nr": "6",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST6"
                  },
                  {
                    "nr": "7",
                    "group": "x87fpu",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "ST7"
                  },
                  {
                    "nr": "0",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX0"
                  },
                  {
                    "nr": "1",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX1"
                  },
                  {
                    "nr": "2",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX2"
                  },
                  {
                    "nr": "3",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX3"
                  },
                  {
                    "nr": "4",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX4"
                  },
                  {
                    "nr": "5",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX5"
                  },
                  {
                    "nr": "6",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX6"
                  },
                  {
                    "nr": "7",
                    "group": "mmx",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "MMX7"
                  },
                  {
                    "nr": "0",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM0"
                  },
                  {
                    "nr": "1",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM1"
                  },
                  {
                    "nr": "2",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM2"
                  },
                  {
                    "nr": "3",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM3"
                  },
                  {
                    "nr": "4",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM4"
                  },
                  {
                    "nr": "5",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM5"
                  },
                  {
                    "nr": "6",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM6"
                  },
                  {
                    "nr": "7",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM7"
                  },
                  {
                    "nr": "8",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM8"
                  },
                  {
                    "nr": "9",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM9"
                  },
                  {
                    "nr": "10",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM10"
                  },
                  {
                    "nr": "11",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM11"
                  },
                  {
                    "nr": "12",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM12"
                  },
                  {
                    "nr": "13",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM13"
                  },
                  {
                    "nr": "14",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM14"
                  },
                  {
                    "nr": "15",
                    "group": "xmm",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "XMM15"
                  }
                ],
                "src": [
                  {
                    "a": "M"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  }
                ]
              },
              "grp1": "system",
              "note": {
                "brief": "Restore Processor Extended States",
                "det": "Restore processor extended states from memory. The states are specified by EDX:EAX"
              }
            },
            {
              "opcd_ext": "6",
              "proc_start": "10",
              "syntax": {
                "mnem": "MFENCE"
              },
              "instr_ext": "sse2",
              "grp1": "order",
              "note": {
                "brief": "Memory Fence"
              }
            },
            {
              "mod": "nomem",
              "opcd_ext": "7",
              "proc_start": "09",
              "syntax": {
                "mnem": "SFENCE"
              },
              "instr_ext": "sse1",
              "grp1": "order",
              "note": {
                "brief": "Store Fence"
              }
            },
            {
              "mod": "mem",
              "opcd_ext": "7",
              "proc_start": "10",
              "syntax": {
                "mnem": "CLFLUSH",
                "src": {
                  "depend": "no",
                  "a": "M",
                  "t": "b"
                }
              },
              "instr_ext": "sse2",
              "grp1": "cachect",
              "note": {
                "brief": "Flush Cache Line"
              }
            }
          ]
        },
        {
          "value": "AF",
          "entry": {
            "direction": "1",
            "op_size": "1",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "IMUL",
              "dst": {
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "arith",
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oc",
            "undef_f": "szap",
            "note": {
              "brief": "Signed Multiply"
            }
          }
        },
        {
          "value": "B0",
          "entry": {
            "direction": "0",
            "op_size": "0",
            "r": "yes",
            "lock": "yes",
            "doc_ref": "gen_note_CMPXCHG_0FB0_0FB1",
            "proc_start": "04",
            "syntax": {
              "mnem": "CMPXCHG",
              "dst": [
                {
                  "a": "E",
                  "t": "b"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "b",
                  "displayed": "no",
                  "#text": "AL"
                }
              ],
              "src": {
                "a": "G",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "arith"
            ],
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare and Exchange"
            }
          }
        },
        {
          "value": "B1",
          "entry": {
            "direction": "0",
            "op_size": "1",
            "r": "yes",
            "lock": "yes",
            "doc_ref": "gen_note_CMPXCHG_0FB0_0FB1",
            "proc_start": "04",
            "syntax": {
              "mnem": "CMPXCHG",
              "dst": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "nr": "0",
                  "group": "gen",
                  "type": "vqp",
                  "displayed": "no",
                  "#text": "rAX"
                }
              ],
              "src": {
                "a": "G",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "arith"
            ],
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Compare and Exchange"
            }
          }
        },
        {
          "value": "B2",
          "entry": {
            "r": "yes",
            "doc64_ref": "gen_note_LSS_0FB2_LFS_0FB4_LGS_0FB5",
            "proc_start": "03",
            "syntax": {
              "mnem": "LSS",
              "dst": [
                {
                  "nr": "2",
                  "group": "seg",
                  "type": "w",
                  "address": "S30",
                  "displayed": "no",
                  "#text": "SS"
                },
                {
                  "a": "G",
                  "t": "vqp"
                }
              ],
              "src": {
                "a": "M",
                "t": "ptp"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "segreg"
            ],
            "note": {
              "brief": "Load Far Pointer"
            }
          }
        },
        {
          "value": "B3",
          "entry": {
            "r": "yes",
            "lock": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "BTR",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "G",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "bit",
            "modif_f": "oszapc",
            "def_f": "c",
            "undef_f": "oszap",
            "note": {
              "brief": "Bit Test and Reset"
            }
          }
        },
        {
          "value": "B4",
          "entry": {
            "r": "yes",
            "doc64_ref": "gen_note_LSS_0FB2_LFS_0FB4_LGS_0FB5",
            "proc_start": "03",
            "syntax": {
              "mnem": "LFS",
              "dst": [
                {
                  "nr": "4",
                  "group": "seg",
                  "type": "w",
                  "address": "S30",
                  "displayed": "no",
                  "#text": "FS"
                },
                {
                  "a": "G",
                  "t": "vqp"
                }
              ],
              "src": {
                "a": "M",
                "t": "ptp"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "segreg"
            ],
            "note": {
              "brief": "Load Far Pointer"
            }
          }
        },
        {
          "value": "B5",
          "entry": {
            "r": "yes",
            "doc64_ref": "gen_note_LSS_0FB2_LFS_0FB4_LGS_0FB5",
            "proc_start": "03",
            "syntax": {
              "mnem": "LGS",
              "dst": [
                {
                  "nr": "5",
                  "group": "seg",
                  "type": "w",
                  "address": "S30",
                  "displayed": "no",
                  "#text": "GS"
                },
                {
                  "a": "G",
                  "t": "vqp"
                }
              ],
              "src": {
                "a": "M",
                "t": "ptp"
              }
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "segreg"
            ],
            "note": {
              "brief": "Load Far Pointer"
            }
          }
        },
        {
          "value": "B6",
          "entry": {
            "direction": "1",
            "op_size": "0",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "MOVZX",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Move with Zero-Extend"
            }
          }
        },
        {
          "value": "B7",
          "entry": {
            "direction": "1",
            "op_size": "1",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "MOVZX",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "w"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Move with Zero-Extend"
            }
          }
        },
        {
          "value": "B8",
          "entry": [
            {
              "proc_start": "99",
              "syntax": {
                "mnem": "JMPE"
              },
              "grp1": "system",
              "grp2": "branch",
              "note": {
                "brief": "Jump to IA-64 Instruction Set"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": {
                "lat_step": "yes",
                "#text": "12"
              },
              "syntax": {
                "mnem": "POPCNT",
                "dst": {
                  "a": "G",
                  "t": "vqp"
                },
                "src": {
                  "a": "E",
                  "t": "vqp"
                }
              },
              "grp1": "gen",
              "grp2": "bit",
              "modif_f": "oszapc",
              "f_vals": "osapc",
              "note": {
                "brief": "Bit Population Count",
                "det": "Count the 1s in op2"
              }
            }
          ]
        },
        {
          "value": "B9",
          "entry": {
            "r": "yes",
            "attr": "invd",
            "doc": "m",
            "doc_ref": "gen_note_sug_UD_0FB9",
            "proc_start": "02",
            "syntax": {
              "mnem": {
                "sug": "yes",
                "#text": "UD"
              },
              "src": [
                {
                  "depend": "no",
                  "a": "G"
                },
                {
                  "depend": "no",
                  "a": "E"
                }
              ]
            },
            "grp1": "gen",
            "grp2": "control",
            "note": {
              "brief": "Undefined Instruction"
            }
          }
        },
        {
          "value": "BA",
          "proc_start": "03",
          "entry": [
            {
              "opcd_ext": "4",
              "syntax": {
                "mnem": "BT",
                "src": [
                  {
                    "a": "E",
                    "t": "vqp"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "grp1": "gen",
              "grp2": "bit",
              "modif_f": "oszapc",
              "def_f": "c",
              "undef_f": "oszap",
              "note": {
                "brief": "Bit Test"
              }
            },
            {
              "lock": "yes",
              "opcd_ext": "5",
              "syntax": {
                "mnem": "BTS",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "grp1": "gen",
              "grp2": "bit",
              "modif_f": "oszapc",
              "def_f": "c",
              "undef_f": "oszap",
              "note": {
                "brief": "Bit Test and Set"
              }
            },
            {
              "lock": "yes",
              "opcd_ext": "6",
              "syntax": {
                "mnem": "BTR",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "grp1": "gen",
              "grp2": "bit",
              "modif_f": "oszapc",
              "def_f": "c",
              "undef_f": "oszap",
              "note": {
                "brief": "Bit Test and Reset"
              }
            },
            {
              "lock": "yes",
              "opcd_ext": "7",
              "syntax": {
                "mnem": "BTC",
                "dst": {
                  "a": "E",
                  "t": "vqp"
                },
                "src": {
                  "a": "I",
                  "t": "b"
                }
              },
              "grp1": "gen",
              "grp2": "bit",
              "modif_f": "oszapc",
              "def_f": "c",
              "undef_f": "oszap",
              "note": {
                "brief": "Bit Test and Complement"
              }
            }
          ]
        },
        {
          "value": "BB",
          "entry": {
            "r": "yes",
            "lock": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "BTC",
              "dst": {
                "a": "E",
                "t": "vqp"
              },
              "src": {
                "a": "G",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "bit",
            "modif_f": "oszapc",
            "def_f": "c",
            "undef_f": "oszap",
            "note": {
              "brief": "Bit Test and Complement"
            }
          }
        },
        {
          "value": "BC",
          "entry": {
            "r": "yes",
            "doc64_ref": "gen_note_BSF_0FBC_BSR_0FBD",
            "proc_start": "03",
            "syntax": {
              "mnem": "BSF",
              "dst": {
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "bit",
            "modif_f": "oszapc",
            "def_f": "z",
            "undef_f": "osapc",
            "note": {
              "brief": "Bit Scan Forward"
            }
          }
        },
        {
          "value": "BD",
          "entry": {
            "r": "yes",
            "doc64_ref": "gen_note_BSF_0FBC_BSR_0FBD",
            "proc_start": "03",
            "syntax": {
              "mnem": "BSR",
              "dst": {
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "bit",
            "modif_f": "oszapc",
            "def_f": "z",
            "undef_f": "osapc",
            "note": {
              "brief": "Bit Scan Reverse"
            }
          }
        },
        {
          "value": "BE",
          "entry": {
            "direction": "1",
            "op_size": "0",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "MOVSX",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "b"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Move with Sign-Extension"
            }
          }
        },
        {
          "value": "BF",
          "entry": {
            "direction": "1",
            "op_size": "1",
            "r": "yes",
            "proc_start": "03",
            "syntax": {
              "mnem": "MOVSX",
              "dst": {
                "depend": "no",
                "a": "G",
                "t": "vqp"
              },
              "src": {
                "a": "E",
                "t": "w"
              }
            },
            "grp1": "gen",
            "grp2": "conver",
            "note": {
              "brief": "Move with Sign-Extension"
            }
          }
        },
        {
          "value": "C0",
          "entry": {
            "direction": "0",
            "op_size": "0",
            "r": "yes",
            "lock": "yes",
            "proc_start": "04",
            "syntax": {
              "mnem": "XADD",
              "dst": [
                {
                  "a": "E",
                  "t": "b"
                },
                {
                  "a": "G",
                  "t": "b"
                }
              ]
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "arith"
            ],
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Exchange and Add"
            }
          }
        },
        {
          "value": "C1",
          "entry": {
            "direction": "0",
            "op_size": "1",
            "r": "yes",
            "lock": "yes",
            "proc_start": "04",
            "syntax": {
              "mnem": "XADD",
              "dst": [
                {
                  "a": "E",
                  "t": "vqp"
                },
                {
                  "a": "G",
                  "t": "vqp"
                }
              ]
            },
            "grp1": "gen",
            "grp2": [
              "datamov",
              "arith"
            ],
            "grp3": "binary",
            "modif_f": "oszapc",
            "def_f": "oszapc",
            "note": {
              "brief": "Exchange and Add"
            }
          }
        },
        {
          "value": "C2",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "CMPPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ps"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "09",
              "syntax": {
                "mnem": "CMPSS",
                "dst": {
                  "a": "V",
                  "t": "ss"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ss"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "compar",
              "note": {
                "brief": "Compare Scalar Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CMPPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "pd"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "compar",
              "note": {
                "brief": "Compare Packed Double-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "CMPSD",
                "dst": {
                  "a": "V",
                  "t": "sd"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "sd"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "compar",
              "note": {
                "brief": "Compare Scalar Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "C3",
          "entry": {
            "r": "yes",
            "proc_start": "10",
            "syntax": {
              "mnem": "MOVNTI",
              "dst": {
                "depend": "no",
                "a": "M",
                "t": "dqp"
              },
              "src": {
                "a": "G",
                "t": "dqp"
              }
            },
            "instr_ext": "sse2",
            "grp1": "cachect",
            "note": {
              "brief": "Store Doubleword Using Non-Temporal Hint"
            }
          }
        },
        {
          "value": "C4",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": [
                {
                  "mod": "nomem",
                  "mnem": "PINSRW",
                  "dst": {
                    "a": "P",
                    "t": "q"
                  },
                  "src": [
                    {
                      "a": "R",
                      "t": "dqp"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mod": "mem",
                  "mnem": "PINSRW",
                  "dst": {
                    "a": "P",
                    "t": "q"
                  },
                  "src": [
                    {
                      "a": "M",
                      "t": "w"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Insert Word"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": [
                {
                  "mod": "nomem",
                  "mnem": "PINSRW",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": [
                    {
                      "a": "R",
                      "t": "dqp"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                },
                {
                  "mod": "mem",
                  "mnem": "PINSRW",
                  "dst": {
                    "a": "V",
                    "t": "dq"
                  },
                  "src": [
                    {
                      "a": "M",
                      "t": "w"
                    },
                    {
                      "a": "I",
                      "t": "b"
                    }
                  ]
                }
              ],
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Insert Word"
              }
            }
          ]
        },
        {
          "value": "C5",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PEXTRW",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": [
                  {
                    "a": "N",
                    "t": "q"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Extract Word"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PEXTRW",
                "dst": {
                  "a": "G",
                  "t": "dqp"
                },
                "src": [
                  {
                    "a": "U",
                    "t": "dq"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Extract Word"
              }
            }
          ]
        },
        {
          "value": "C6",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "SHUFPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "ps"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse1",
              "grp1": "simdfp",
              "grp2": "shunpck",
              "note": {
                "brief": "Shuffle Packed Single-FP Values"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "SHUFPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": [
                  {
                    "a": "W",
                    "t": "pd"
                  },
                  {
                    "a": "I",
                    "t": "b"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "shunpck",
              "note": {
                "brief": "Shuffle Packed Double-FP Values"
              }
            }
          ]
        },
        {
          "value": "C7",
          "entry": [
            {
              "lock": "yes",
              "doc_ref": "gen_note_CMPXCHG8B_CMPXCHG16B_0FC7_1",
              "opcd_ext": "1",
              "proc_start": "05",
              "syntax": {
                "mnem": "CMPXCHG8B",
                "dst": [
                  {
                    "a": "M",
                    "t": "q"
                  },
                  {
                    "nr": "0",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EAX"
                  },
                  {
                    "nr": "2",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EDX"
                  }
                ],
                "src": [
                  {
                    "nr": "3",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "EBX"
                  },
                  {
                    "nr": "1",
                    "group": "gen",
                    "type": "d",
                    "displayed": "no",
                    "#text": "ECX"
                  }
                ]
              },
              "grp1": "gen",
              "grp2": [
                "datamov",
                "arith"
              ],
              "grp3": "binary",
              "modif_f": "z",
              "def_f": "z",
              "note": {
                "brief": "Compare and Exchange Bytes"
              }
            },
            {
              "lock": "yes",
              "doc64_ref": "gen_note_CMPXCHG8B_CMPXCHG16B_0FC7_1",
              "mode": "e",
              "opcd_ext": "1",
              "proc_start": "10",
              "syntax": [
                {
                  "mnem": "CMPXCHG8B",
                  "dst": [
                    {
                      "a": "M",
                      "t": "q"
                    },
                    {
                      "nr": "0",
                      "group": "gen",
                      "type": "d",
                      "displayed": "no",
                      "#text": "EAX"
                    },
                    {
                      "nr": "2",
                      "group": "gen",
                      "type": "d",
                      "displayed": "no",
                      "#text": "EDX"
                    }
                  ],
                  "src": [
                    {
                      "nr": "3",
                      "group": "gen",
                      "type": "d",
                      "displayed": "no",
                      "#text": "EBX"
                    },
                    {
                      "nr": "1",
                      "group": "gen",
                      "type": "d",
                      "displayed": "no",
                      "#text": "ECX"
                    }
                  ]
                },
                {
                  "mnem": "CMPXCHG16B",
                  "dst": [
                    {
                      "a": "M",
                      "t": "dq"
                    },
                    {
                      "nr": "0",
                      "group": "gen",
                      "type": "qp",
                      "displayed": "no",
                      "#text": "RAX"
                    },
                    {
                      "nr": "2",
                      "group": "gen",
                      "type": "qp",
                      "displayed": "no",
                      "#text": "RDX"
                    }
                  ],
                  "src": [
                    {
                      "nr": "3",
                      "group": "gen",
                      "type": "qp",
                      "displayed": "no",
                      "#text": "RBX"
                    },
                    {
                      "nr": "1",
                      "group": "gen",
                      "type": "qp",
                      "displayed": "no",
                      "#text": "RCX"
                    }
                  ]
                }
              ],
              "grp1": "gen",
              "grp2": [
                "datamov",
                "arith"
              ],
              "grp3": "binary",
              "modif_f": "z",
              "def_f": "z",
              "note": {
                "brief": "Compare and Exchange Bytes"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "6",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMPTRLD",
                "src": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Load Pointer to Virtual-Machine Control Structure",
                "det": "Loads the current VMCS pointer from memory"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "pref": "66",
              "opcd_ext": "6",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMCLEAR",
                "dst": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Clear Virtual-Machine Control Structure",
                "det": "Copy VMCS data to VMCS region in memory"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "pref": "F3",
              "opcd_ext": "6",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMXON",
                "src": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Enter VMX Operation",
                "det": "Enter VMX root operation"
              }
            },
            {
              "doc_ref": "gen_note_VMX_vs_SVM",
              "ring": "0",
              "mode": "p",
              "opcd_ext": "7",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "VMPTRST",
                "dst": {
                  "a": "M",
                  "t": "q"
                }
              },
              "instr_ext": "vmx",
              "modif_f": "oszapc",
              "def_f": "oszapc",
              "note": {
                "brief": "Store Pointer to Virtual-Machine Control Structure",
                "det": "Stores the current VMCS pointer into memory"
              }
            }
          ]
        },
        {
          "value": "C8",
          "entry": {
            "doc_ref": "gen_note_BSWAP_0FC8",
            "proc_start": "04",
            "syntax": {
              "mnem": "BSWAP",
              "dst": {
                "a": "Z",
                "t": "vqp"
              }
            },
            "grp1": "gen",
            "grp2": "datamov",
            "note": {
              "brief": "Byte Swap"
            }
          }
        },
        {
          "value": "D0",
          "entry": [
            {
              "r": "yes",
              "pref": "66",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "ADDSUBPD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Packed Double-FP Add/Subtract"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": {
                "lat_step": "yes",
                "#text": "10"
              },
              "syntax": {
                "mnem": "ADDSUBPS",
                "dst": {
                  "a": "V",
                  "t": "ps"
                },
                "src": {
                  "a": "W",
                  "t": "ps"
                }
              },
              "instr_ext": "sse3",
              "grp1": "simdfp",
              "grp2": "arith",
              "note": {
                "brief": "Packed Single-FP Add/Subtract"
              }
            }
          ]
        },
        {
          "value": "D1",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRLW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            }
          ]
        },
        {
          "value": "D2",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRLD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            }
          ]
        },
        {
          "value": "D3",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRLQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRLQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Right Logical"
              }
            }
          ]
        },
        {
          "value": "D4",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Quadword Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Quadword Integers"
              }
            }
          ]
        },
        {
          "value": "D5",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PMULLW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Multiply Packed Signed Integers and Store Low Result"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PMULLW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Signed Integers and Store Low Result"
              }
            }
          ]
        },
        {
          "value": "D6",
          "entry": [
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVQ",
                "dst": {
                  "a": "W",
                  "t": "q"
                },
                "src": {
                  "a": "V",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVQ2DQ",
                "dst": {
                  "depend": "no",
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "N",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Quadword from MMX Technology to XMM Register"
              }
            },
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVDQ2Q",
                "dst": {
                  "depend": "no",
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "U",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "datamov",
              "note": {
                "brief": "Move Quadword from XMM to MMX Technology Register"
              }
            }
          ]
        },
        {
          "value": "D7",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMOVMSKB",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "N",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Move Byte Mask"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMOVMSKB",
                "dst": {
                  "depend": "no",
                  "a": "G",
                  "t": "dqp"
                },
                "src": {
                  "a": "U",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Move Byte Mask"
              }
            }
          ]
        },
        {
          "value": "D8",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBUSB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Unsigned Integers with Unsigned Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBUSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Unsigned Integers with Unsigned Saturation"
              }
            }
          ]
        },
        {
          "value": "D9",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBUSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Unsigned Integers with Unsigned Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBUSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Unsigned Integers with Unsigned Saturation"
              }
            }
          ]
        },
        {
          "value": "DA",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMINUB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Minimum of Packed Unsigned Byte Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMINUB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Minimum of Packed Unsigned Byte Integers"
              }
            }
          ]
        },
        {
          "value": "DB",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PAND",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "logical",
              "note": {
                "brief": "Logical AND"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PAND",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "logical",
              "note": {
                "brief": "Logical AND"
              }
            }
          ]
        },
        {
          "value": "DC",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDUSB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Unsigned Integers with Unsigned Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDUSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Unsigned Integers with Unsigned Saturation"
              }
            }
          ]
        },
        {
          "value": "DD",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDUSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Unsigned Integers with Unsigned Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDUSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Unsigned Integers with Unsigned Saturation"
              }
            }
          ]
        },
        {
          "value": "DE",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMAXUB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Maximum of Packed Unsigned Byte Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMAXUB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Maximum of Packed Unsigned Byte Integers"
              }
            }
          ]
        },
        {
          "value": "DF",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PANDN",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "logical",
              "note": {
                "brief": "Logical AND NOT"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PANDN",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "logical",
              "note": {
                "brief": "Logical AND NOT"
              }
            }
          ]
        },
        {
          "value": "E0",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PAVGB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Average Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PAVGB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Average Packed Integers"
              }
            }
          ]
        },
        {
          "value": "E1",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRAW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRAW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            }
          ]
        },
        {
          "value": "E2",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSRAD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSRAD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Right Arithmetic"
              }
            }
          ]
        },
        {
          "value": "E3",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PAVGW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Average Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PAVGW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Average Packed Integers"
              }
            }
          ]
        },
        {
          "value": "E4",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMULHUW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Multiply Packed Unsigned Integers and Store High Result"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMULHUW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Multiply Packed Unsigned Integers and Store High Result"
              }
            }
          ]
        },
        {
          "value": "E5",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PMULHW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Multiply Packed Signed Integers and Store High Result"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PMULHW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Signed Integers and Store High Result"
              }
            }
          ]
        },
        {
          "value": "E6",
          "entry": [
            {
              "r": "yes",
              "pref": "F2",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTPD2DQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert Packed Double-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTTPD2DQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "pd"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert with Trunc. Packed Double-FP Values to",
                  "DW Integers"
                ]
              }
            },
            {
              "r": "yes",
              "pref": "F3",
              "proc_start": "10",
              "syntax": {
                "mnem": "CVTDQ2PD",
                "dst": {
                  "a": "V",
                  "t": "pd"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "pcksclr",
              "grp2": "conver",
              "note": {
                "brief": [
                  "Convert Packed DW Integers to",
                  "Double-FP Values"
                ]
              }
            }
          ]
        },
        {
          "value": "E7",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "MOVNTQ",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "q"
                },
                "src": {
                  "a": "P",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "cachect",
              "note": {
                "brief": "Store of Quadword Using Non-Temporal Hint"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MOVNTDQ",
                "dst": {
                  "depend": "no",
                  "a": "M",
                  "t": "dq"
                },
                "src": {
                  "a": "V",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "cachect",
              "note": {
                "brief": "Store Double Quadword Using Non-Temporal Hint"
              }
            }
          ]
        },
        {
          "value": "E8",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBSB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Signed Integers with Signed Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Signed Integers with Signed Saturation"
              }
            }
          ]
        },
        {
          "value": "E9",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Signed Integers with Signed Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Signed Integers with Signed Saturation"
              }
            }
          ]
        },
        {
          "value": "EA",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMINSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Minimum of Packed Signed Word Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMINSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Minimum of Packed Signed Word Integers"
              }
            }
          ]
        },
        {
          "value": "EB",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "POR",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "logical",
              "note": {
                "brief": "Bitwise Logical OR"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "POR",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "logical",
              "note": {
                "brief": "Bitwise Logical OR"
              }
            }
          ]
        },
        {
          "value": "EC",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDSB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Signed Integers with Signed Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDSB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Signed Integers with Signed Saturation"
              }
            }
          ]
        },
        {
          "value": "ED",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Signed Integers with Signed Saturation"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Signed Integers with Signed Saturation"
              }
            }
          ]
        },
        {
          "value": "EE",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMAXSW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Maximum of Packed Signed Word Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PMAXSW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Maximum of Packed Signed Word Integers"
              }
            }
          ]
        },
        {
          "value": "EF",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PXOR",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "logical",
              "note": {
                "brief": "Logical Exclusive OR"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PXOR",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "logical",
              "note": {
                "brief": "Logical Exclusive OR"
              }
            }
          ]
        },
        {
          "value": "F0",
          "entry": {
            "r": "yes",
            "pref": "F2",
            "proc_start": {
              "lat_step": "yes",
              "#text": "10"
            },
            "syntax": {
              "mnem": "LDDQU",
              "dst": {
                "depend": "no",
                "a": "V",
                "t": "dq"
              },
              "src": {
                "a": "M",
                "t": "dq"
              }
            },
            "instr_ext": "sse3",
            "grp1": "cachect",
            "note": {
              "brief": "Load Unaligned Integer 128 Bits"
            }
          }
        },
        {
          "value": "F1",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSLLW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            }
          ]
        },
        {
          "value": "F2",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSLLD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            }
          ]
        },
        {
          "value": "F3",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSLLQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSLLQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "shift",
              "note": {
                "brief": "Shift Packed Data Left Logical"
              }
            }
          ]
        },
        {
          "value": "F4",
          "entry": [
            {
              "r": "yes",
              "proc_start": "10",
              "syntax": {
                "mnem": "PMULUDQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Unsigned DW Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PMULUDQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply Packed Unsigned DW Integers"
              }
            }
          ]
        },
        {
          "value": "F5",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PMADDWD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "d"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Multiply and Add Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PMADDWD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Multiply and Add Packed Integers"
              }
            }
          ]
        },
        {
          "value": "F6",
          "entry": [
            {
              "r": "yes",
              "proc_start": "09",
              "syntax": {
                "mnem": "PSADBW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Compute Sum of Absolute Differences"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "09",
              "syntax": {
                "mnem": "PSADBW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse1",
              "grp1": "simdint",
              "note": {
                "brief": "Compute Sum of Absolute Differences"
              }
            }
          ]
        },
        {
          "value": "F7",
          "entry": [
            {
              "r": "yes",
              "doc_ref": "gen_note_MASKMOVQ_0FF7",
              "proc_start": "09",
              "syntax": {
                "mnem": "MASKMOVQ",
                "dst": [
                  {
                    "type": "q",
                    "address": "BD",
                    "depend": "no",
                    "displayed": "no",
                    "#text": "(DS:)[rDI]"
                  },
                  {
                    "a": "P",
                    "t": "q"
                  }
                ],
                "src": {
                  "a": "N",
                  "t": "q"
                }
              },
              "instr_ext": "sse1",
              "grp1": "cachect",
              "note": {
                "brief": "Store Selected Bytes of Quadword"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "MASKMOVDQU",
                "dst": {
                  "type": "dq",
                  "address": "BD",
                  "depend": "no",
                  "displayed": "no",
                  "#text": "(DS:)[rDI]"
                },
                "src": [
                  {
                    "a": "V",
                    "t": "dq"
                  },
                  {
                    "a": "U",
                    "t": "dq"
                  }
                ]
              },
              "instr_ext": "sse2",
              "grp1": "cachect",
              "note": {
                "brief": "Store Selected Bytes of Double Quadword"
              }
            }
          ]
        },
        {
          "value": "F8",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Integers"
              }
            }
          ]
        },
        {
          "value": "F9",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Integers"
              }
            }
          ]
        },
        {
          "value": "FA",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PSUBD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Subtract Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Integers"
              }
            }
          ]
        },
        {
          "value": "FB",
          "entry": [
            {
              "r": "yes",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBQ",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Quadword Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PSUBQ",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Subtract Packed Quadword Integers"
              }
            }
          ]
        },
        {
          "value": "FC",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDB",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDB",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Integers"
              }
            }
          ]
        },
        {
          "value": "FD",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDW",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDW",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Integers"
              }
            }
          ]
        },
        {
          "value": "FE",
          "entry": [
            {
              "r": "yes",
              "proc_start": "06",
              "syntax": {
                "mnem": "PADDD",
                "dst": {
                  "a": "P",
                  "t": "q"
                },
                "src": {
                  "a": "Q",
                  "t": "q"
                }
              },
              "instr_ext": "mmx",
              "grp1": "arith",
              "note": {
                "brief": "Add Packed Integers"
              }
            },
            {
              "r": "yes",
              "pref": "66",
              "proc_start": "10",
              "syntax": {
                "mnem": "PADDD",
                "dst": {
                  "a": "V",
                  "t": "dq"
                },
                "src": {
                  "a": "W",
                  "t": "dq"
                }
              },
              "instr_ext": "sse2",
              "grp1": "simdint",
              "grp2": "arith",
              "note": {
                "brief": "Add Packed Integers"
              }
            }
          ]
        }
      ]
    },
    "gen_notes": [
      {
        "id": "gen_note_opcd_POP_CS_0F"
      },
      {
        "id": "gen_note_branch_prefixes"
      },
      {
        "id": "gen_note_plain_F390"
      },
      {
        "id": "gen_note_SAHF_9E_LAHF_9F"
      },
      {
        "id": "gen_note_SAL_C0_4_C1_4_D0_4_D1_4"
      },
      {
        "id": "gen_note_undefined_D6_F1"
      },
      {
        "id": "gen_note_u_SALC_D6"
      },
      {
        "id": "gen_note_FSTP1_D9_3_FSTP8_DF_2_FSTP9_DF_3"
      },
      {
        "id": "gen_note_FSTP1_D9_3_not_true_alias"
      },
      {
        "id": "gen_note_FNENI_DBE0_FNDISI_DBE1"
      },
      {
        "id": "gen_note_FNSETPM_DBE4"
      },
      {
        "id": "gen_note_FCOM2_DC_2"
      },
      {
        "id": "gen_note_FCOMP3_DC_3_FCOMP5_DE_2"
      },
      {
        "id": "gen_note_FXCH4_DD_1_FXCH7_DF_1"
      },
      {
        "id": "gen_note_FFREEP_DF_1"
      },
      {
        "id": "gen_note_x87_fpu_undoc_aliases"
      },
      {
        "id": "gen_note_u_INT1_ICEBP_F1"
      },
      {
        "id": "gen_note_REP_F2_F3"
      },
      {
        "id": "gen_note_TEST_F6_1_F7_1"
      },
      {
        "id": "gen_note_CALLF_FF_3_JMPF_FF_5"
      },
      {
        "id": "gen_note_SMSW_0F01_4"
      },
      {
        "id": "gen_note_u_LOADALL_0F05_0F07"
      },
      {
        "id": "gen_note_SYSCALL_0F05"
      },
      {
        "id": "gen_note_NOP_0F0D"
      },
      {
        "id": "gen_note_MOV_CR_0F20_0F22"
      },
      {
        "id": "gen_note_u_MOV_CR_DR_TR_0F20_0F21_0F22_0F23_0F24_0F26"
      },
      {
        "id": "gen_note_SYSENTER_0F34"
      },
      {
        "id": "gen_note_SYSEXIT_0F35"
      },
      {
        "id": "gen_note_GETSEC_0F37"
      },
      {
        "id": "gen_note_MOVQ_0F6E_660F6E_0F7E_660F7E"
      },
      {
        "id": "gen_note_SETcc_0F90-0F9F"
      },
      {
        "id": "gen_note_CMPXCHG_0FB0_0FB1"
      },
      {
        "id": "gen_note_LSS_0FB2_LFS_0FB4_LGS_0FB5"
      },
      {
        "id": "gen_note_sug_UD_0FB9"
      },
      {
        "id": "gen_note_BSF_0FBC_BSR_0FBD"
      },
      {
        "id": "gen_note_CMPXCHG8B_CMPXCHG16B_0FC7_1"
      },
      {
        "id": "gen_note_BSWAP_0FC8"
      },
      {
        "id": "gen_note_short_near_jmp"
      },
      {
        "id": "gen_note_VMX_vs_SVM"
      },
      {
        "id": "gen_note_SSE4_amd"
      }
    ],
    "ring_notes": [
      {
        "id": "rflags_iopl"
      },
      {
        "id": "cr4_tsd"
      },
      {
        "id": "cr4_pce"
      }
    ]
  }
"###;