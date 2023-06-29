pub mod builder;
pub mod layer;
pub mod named_graph;
pub mod store;
pub mod value;

pub use swipl;
pub use terminus_store;

pub fn install(module: Option<&str>) {
    store::register_open_memory_store_in_module(module);
    store::register_open_directory_store_in_module(module);
    store::register_open_raw_archive_store_in_module(module);
    store::register_open_archive_store_in_module(module);
    store::register_open_grpc_store_in_module(module);
    named_graph::register_create_named_graph_in_module(module);
    named_graph::register_open_named_graph_in_module(module);
    named_graph::register_delete_named_graph_in_module(module);
    named_graph::register_head2_in_module(module);
    named_graph::register_head3_in_module(module);
    named_graph::register_nb_set_head_in_module(module);
    named_graph::register_nb_force_set_head_in_module(module);
    named_graph::register_nb_force_set_head_version_in_module(module);
    store::register_open_write_in_module(module);
    store::register_merge_base_layers_in_module(module);
    builder::register_nb_add_id_triple_in_module(module);
    builder::register_nb_add_object_triple_in_module(module);
    builder::register_nb_remove_id_triple_in_module(module);
    builder::register_nb_remove_object_triple_in_module(module);
    builder::register_builder_committed_in_module(module);
    builder::register_nb_commit_in_module(module);
    builder::register_nb_apply_delta_in_module(module);
    builder::register_nb_apply_diff_in_module(module);
    layer::register_node_and_value_count_in_module(module);
    layer::register_predicate_count_in_module(module);
    layer::register_subject_to_id_in_module(module);
    layer::register_id_to_subject_in_module(module);
    layer::register_predicate_to_id_in_module(module);
    layer::register_id_to_predicate_in_module(module);
    layer::register_object_to_id_in_module(module);
    layer::register_id_to_object_in_module(module);
    layer::register_parent_in_module(module);
    layer::register_squash_in_module(module);
    layer::register_squash_upto_in_module(module);
    layer::register_rollup_in_module(module);
    layer::register_rollup_upto_in_module(module);
    layer::register_imprecise_rollup_upto_in_module(module);
    layer::register_layer_addition_count_in_module(module);
    layer::register_layer_removal_count_in_module(module);
    layer::register_layer_total_addition_count_in_module(module);
    layer::register_layer_total_removal_count_in_module(module);
    layer::register_layer_total_triple_count_in_module(module);
    layer::register_retrieve_layer_stack_names_in_module(module);
    layer::register_layer_to_id_in_module(module);
    layer::register_store_id_layer_in_module(module);
    layer::register_layer_equals_in_module(module);
    store::register_pack_export_in_module(module);
    store::register_pack_layerids_and_parents_in_module(module);
    store::register_pack_import_in_module(module);
    layer::register_id_triple_in_module(module);
    layer::register_id_triple_addition_in_module(module);
    layer::register_id_triple_removal_in_module(module);
    layer::register_sp_card_in_module(module);
    layer::register_op_card_in_module(module);
}
