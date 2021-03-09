#[macro_use]
extern crate flatdata;

include!(concat!(env!("OUT_DIR"), "/wasmer_cache.rs"));

#[test]
fn test() {
    let storage = flatdata::FileResourceStorage::new("/tmp/flatdata_test");
    let builder = wasmer_cache::CompiledFunctionFrameInfoBuilder::new(storage.clone())
        .expect("Failed to create builder");
    let fam_builder = builder.address_map().expect("Failed to create builder");
    let mut faml = wasmer_cache::FunctionAddressMapLocation::new();
    faml.set_start_srcloc(14);
    faml.set_end_srcloc(10);
    faml.set_body_offset(1);
    faml.set_body_len(6);
    fam_builder.set_loc(&faml).expect("Failed to set loc");

    let mut instructions = flatdata::Vector::<wasmer_cache::InstructionAddressMap>::new();
    let iam = instructions.grow();
    iam.set_srcloc(5);
    iam.set_code_offset(7);
    iam.set_code_len(8);
    fam_builder
        .set_instructions(&instructions)
        .expect("Failed to set instructions");

    let mut traps = flatdata::Vector::<wasmer_cache::TrapInformation>::new();
    let ti = traps.grow();
    ti.set_code_offset(1);
    ti.set_trap_code(wasmer_cache::TrapCode::Stackoverflow);
    builder.set_traps(&traps).expect("Failed to set trap");

    let cffi =
        wasmer_cache::CompiledFunctionFrameInfo::open(storage).expect("Failed to open archive");
    assert_eq!(cffi.address_map().loc().start_srcloc(), 14);
    assert_eq!(cffi.address_map().loc().body_offset(), 1);
    assert_eq!(cffi.address_map().instructions()[0].srcloc(), 5);
}
