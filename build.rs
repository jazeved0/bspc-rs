const SOURCES: &[&str] = &[
    "src/c/aas_areamerging.c",
    "src/c/aas_cfg.c",
    "src/c/aas_create.c",
    "src/c/aas_edgemelting.c",
    "src/c/aas_facemerging.c",
    "src/c/aas_file.c",
    "src/c/aas_gsubdiv.c",
    "src/c/aas_map.c",
    "src/c/aas_prunenodes.c",
    "src/c/aas_store.c",
    "src/c/be_aas_bspc.c",
    "src/c/deps/botlib/be_aas_bspq3.c",
    "src/c/deps/botlib/be_aas_cluster.c",
    "src/c/deps/botlib/be_aas_move.c",
    "src/c/deps/botlib/be_aas_optimize.c",
    "src/c/deps/botlib/be_aas_reach.c",
    "src/c/deps/botlib/be_aas_sample.c",
    "src/c/brushbsp.c",
    "src/c/bspc.c",
    "src/c/deps/qcommon/cm_load.c",
    "src/c/deps/qcommon/cm_patch.c",
    "src/c/deps/qcommon/cm_test.c",
    "src/c/deps/qcommon/cm_trace.c",
    "src/c/csg.c",
    "src/c/glfile.c",
    "src/c/l_bsp_ent.c",
    "src/c/l_bsp_hl.c",
    "src/c/l_bsp_q1.c",
    "src/c/l_bsp_q2.c",
    "src/c/l_bsp_q3.c",
    "src/c/l_bsp_sin.c",
    "src/c/l_cmd.c",
    "src/c/deps/botlib/l_libvar.c",
    "src/c/l_log.c",
    "src/c/l_math.c",
    "src/c/l_mem.c",
    "src/c/l_poly.c",
    "src/c/deps/botlib/l_precomp.c",
    "src/c/l_qfiles.c",
    "src/c/deps/botlib/l_script.c",
    "src/c/deps/botlib/l_struct.c",
    "src/c/l_threads.c",
    "src/c/l_utils.c",
    "src/c/leakfile.c",
    "src/c/map.c",
    "src/c/map_hl.c",
    "src/c/map_q1.c",
    "src/c/map_q2.c",
    "src/c/map_q3.c",
    "src/c/map_sin.c",
    "src/c/deps/qcommon/md4.c",
    "src/c/nodraw.c",
    "src/c/portals.c",
    "src/c/textures.c",
    "src/c/tree.c",
    "src/c/deps/qcommon/unzip.c",
];

const INCLUDES: &[&str] = &["src/c", "src/c/deps"];

fn main() {
    let mut cfg = cc::Build::new();
    for source in SOURCES {
        cfg.file(source);
        println!("cargo:rerun-if-changed={source}");
    }
    for include in INCLUDES {
        cfg.include(include);
    }
    cfg.define("stricmp", "strcasecmp");
    cfg.define("Com_Memcpy", "memcpy");
    cfg.define("Com_Memset", "memset");
    cfg.define("MAC_STATIC", "");
    cfg.define("QDECL", "");
    cfg.define("LINUX", None);
    cfg.define("BSPC", None);
    cfg.define("_FORTIFY_SOURCE", "2");
    cfg.flag("-ffast-math");
    
    // Suppress all warnings:
    cfg.warnings(false);
    cfg.flag("-w");
    
    cfg.compile("libbspc.a");
}
