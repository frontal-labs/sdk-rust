//! Service scoped access for the `ontology` API.

use serde::{de::DeserializeOwned, Serialize};

use super::ServiceClient;
use crate::{encode_path_segment, global::Result};

/// Type marker for the `ontology` service namespace.
#[derive(Clone, Copy, Debug)]
pub struct OntologyService;

/// Typed access to the `ontology` service.
pub type Ontology = ServiceClient<OntologyService>;

/// Service key used by the contract gate.
pub const SERVICE_NAME: &str = "ontology";

macro_rules! get_query_operation {
    ($name:ident, $doc:literal, $path:expr) => {
        #[doc = $doc]
        pub async fn $name<T>(&self, query: &[(&str, &str)]) -> Result<T>
        where
            T: DeserializeOwned,
        {
            self.get_with_query($path, query).await
        }
    };
}

macro_rules! get_id_operation {
    ($name:ident, $doc:literal, |$id:ident| $path:expr) => {
        #[doc = $doc]
        pub async fn $name<T>(&self, $id: &str) -> Result<T>
        where
            T: DeserializeOwned,
        {
            self.get($path).await
        }
    };
}

macro_rules! post_operation {
    ($name:ident, $doc:literal, $path:expr) => {
        #[doc = $doc]
        pub async fn $name<B, T>(&self, request: &B) -> Result<T>
        where
            B: Serialize + ?Sized,
            T: DeserializeOwned,
        {
            self.post($path, request).await
        }
    };
}

macro_rules! post_id_operation {
    ($name:ident, $doc:literal, |$id:ident| $path:expr) => {
        #[doc = $doc]
        pub async fn $name<B, T>(&self, $id: &str, request: &B) -> Result<T>
        where
            B: Serialize + ?Sized,
            T: DeserializeOwned,
        {
            self.post($path, request).await
        }
    };
}

macro_rules! put_id_operation {
    ($name:ident, $doc:literal, |$id:ident| $path:expr) => {
        #[doc = $doc]
        pub async fn $name<B, T>(&self, $id: &str, request: &B) -> Result<T>
        where
            B: Serialize + ?Sized,
            T: DeserializeOwned,
        {
            self.put($path, request).await
        }
    };
}

macro_rules! delete_id_operation {
    ($name:ident, $doc:literal, |$id:ident| $path:expr) => {
        #[doc = $doc]
        pub async fn $name(&self, $id: &str) -> Result<()> {
            self.delete_empty($path).await
        }
    };
}

impl ServiceClient<OntologyService> {
    get_query_operation!(
        list_events,
        "List ontology events.",
        "ontology/events/events"
    );
    get_id_operation!(get_event, "Fetch an ontology event.", |id| format!(
        "ontology/events/events/{}",
        encode_path_segment(id)
    ));
    post_operation!(
        create_event,
        "Create an ontology event.",
        "ontology/events/events"
    );
    post_operation!(
        create_checkpoint,
        "Create an ontology event checkpoint.",
        "ontology/events/events/checkpoints"
    );
    get_id_operation!(
        get_checkpoint,
        "Fetch an ontology event checkpoint for a consumer.",
        |id| format!(
            "ontology/events/events/checkpoints/{}",
            encode_path_segment(id)
        )
    );
    post_operation!(
        acquire_event_lease,
        "Acquire an ontology event lease.",
        "ontology/events/events/leases/acquire"
    );
    post_operation!(
        acknowledge_event_lease,
        "Acknowledge an ontology event lease.",
        "ontology/events/events/leases/acknowledge"
    );

    get_query_operation!(
        list_object_types,
        "List ontology object types.",
        "ontology/objects/object-types"
    );
    get_id_operation!(
        get_object_type,
        "Fetch an ontology object type.",
        |id| format!("ontology/objects/object-types/{}", encode_path_segment(id))
    );
    put_id_operation!(
        put_object_type,
        "Create or replace an ontology object type.",
        |id| format!("ontology/objects/object-types/{}", encode_path_segment(id))
    );
    delete_id_operation!(
        delete_object_type,
        "Delete an ontology object type.",
        |id| format!("ontology/objects/object-types/{}", encode_path_segment(id))
    );
    get_query_operation!(
        list_objects,
        "List ontology objects.",
        "ontology/objects/objects"
    );
    get_id_operation!(get_object, "Fetch an ontology object.", |id| format!(
        "ontology/objects/objects/{}",
        encode_path_segment(id)
    ));
    put_id_operation!(
        put_object,
        "Create or replace an ontology object.",
        |id| format!("ontology/objects/objects/{}", encode_path_segment(id))
    );
    delete_id_operation!(delete_object, "Delete an ontology object.", |id| format!(
        "ontology/objects/objects/{}",
        encode_path_segment(id)
    ));

    get_query_operation!(
        list_relationship_types,
        "List ontology relationship types.",
        "ontology/relationships/relationship-types"
    );
    delete_id_operation!(
        delete_relationship_type,
        "Delete an ontology relationship type.",
        |id| format!(
            "ontology/relationships/relationship-types/{}",
            encode_path_segment(id)
        )
    );
    get_query_operation!(
        list_relationships,
        "List ontology relationships.",
        "ontology/relationships/relationships"
    );
    get_id_operation!(
        get_relationship,
        "Fetch an ontology relationship.",
        |id| format!(
            "ontology/relationships/relationships/{}",
            encode_path_segment(id)
        )
    );
    put_id_operation!(
        put_relationship,
        "Create or replace an ontology relationship.",
        |id| format!(
            "ontology/relationships/relationships/{}",
            encode_path_segment(id)
        )
    );
    delete_id_operation!(
        delete_relationship,
        "Delete an ontology relationship.",
        |id| format!(
            "ontology/relationships/relationships/{}",
            encode_path_segment(id)
        )
    );

    get_query_operation!(
        list_schemas,
        "List ontology schemas.",
        "ontology/schemas/schemas"
    );
    post_operation!(
        create_schema,
        "Create an ontology schema.",
        "ontology/schemas/schemas"
    );
    post_operation!(
        validate_schema,
        "Validate an ontology schema.",
        "ontology/schemas/schemas/validate"
    );
    get_id_operation!(get_schema, "Fetch an ontology schema.", |id| format!(
        "ontology/schemas/schemas/{}",
        encode_path_segment(id)
    ));
    delete_id_operation!(delete_schema, "Delete an ontology schema.", |id| format!(
        "ontology/schemas/schemas/{}",
        encode_path_segment(id)
    ));

    post_operation!(
        create_version,
        "Create an ontology version.",
        "ontology/versions/versions"
    );
    post_operation!(
        compare_versions,
        "Compare ontology versions.",
        "ontology/versions/versions/compare"
    );
    get_id_operation!(get_version, "Fetch an ontology version.", |id| format!(
        "ontology/versions/versions/{}",
        encode_path_segment(id)
    ));
    delete_id_operation!(delete_version, "Delete an ontology version.", |id| format!(
        "ontology/versions/versions/{}",
        encode_path_segment(id)
    ));
    post_operation!(
        verify_audit_chain,
        "Verify the ontology audit chain.",
        "ontology/versions/audit/verify"
    );
    get_query_operation!(
        list_release_bundles,
        "List ontology release bundles.",
        "ontology/versions/release-bundles"
    );
    post_operation!(
        create_release_bundle,
        "Create an ontology release bundle.",
        "ontology/versions/release-bundles"
    );
    get_id_operation!(
        get_release_bundle,
        "Fetch an ontology release bundle.",
        |id| format!(
            "ontology/versions/release-bundles/{}",
            encode_path_segment(id)
        )
    );

    post_operation!(
        validate_payload,
        "Validate a payload against ontology rules.",
        "ontology/validation/payloads/validate"
    );
    get_query_operation!(
        list_validation_rules,
        "List ontology validation rules.",
        "ontology/validation/rules"
    );
    post_operation!(
        create_validation_rule,
        "Create an ontology validation rule.",
        "ontology/validation/rules"
    );
    get_id_operation!(
        get_validation_rule,
        "Fetch an ontology validation rule.",
        |id| format!("ontology/validation/rules/{}", encode_path_segment(id))
    );
    delete_id_operation!(
        delete_validation_rule,
        "Delete an ontology validation rule.",
        |id| format!("ontology/validation/rules/{}", encode_path_segment(id))
    );

    post_operation!(
        create_transformation,
        "Create an ontology transformation.",
        "ontology/transformations/transformations"
    );

    post_operation!(
        explain_reasoning,
        "Explain an ontology reasoning result.",
        "ontology/reasoning/explain"
    );
    post_operation!(
        query_facts,
        "Query ontology facts.",
        "ontology/reasoning/facts"
    );
    post_operation!(
        load_facts_graph,
        "Load a graph of ontology facts.",
        "ontology/reasoning/facts/load-graph"
    );
    post_operation!(
        reason_forward,
        "Run forward ontology reasoning.",
        "ontology/reasoning/reason/forward"
    );
    post_operation!(
        reason_backward,
        "Run backward ontology reasoning.",
        "ontology/reasoning/reason/backward"
    );
    get_query_operation!(
        list_reasoning_rules,
        "List ontology reasoning rules.",
        "ontology/reasoning/rules"
    );
    post_operation!(
        create_reasoning_rule,
        "Create an ontology reasoning rule.",
        "ontology/reasoning/rules"
    );
    put_id_operation!(
        update_reasoning_rule,
        "Update an ontology reasoning rule.",
        |id| format!("ontology/reasoning/rules/{}", encode_path_segment(id))
    );
    delete_id_operation!(
        delete_reasoning_rule,
        "Delete an ontology reasoning rule.",
        |id| format!("ontology/reasoning/rules/{}", encode_path_segment(id))
    );

    get_query_operation!(
        list_rollouts,
        "List ontology rollouts.",
        "ontology/rollouts/rollouts"
    );
    post_operation!(
        create_rollout,
        "Create an ontology rollout.",
        "ontology/rollouts/rollouts"
    );
    get_id_operation!(get_rollout, "Fetch an ontology rollout.", |id| format!(
        "ontology/rollouts/rollouts/{}",
        encode_path_segment(id)
    ));
    put_id_operation!(update_rollout, "Update an ontology rollout.", |id| format!(
        "ontology/rollouts/rollouts/{}",
        encode_path_segment(id)
    ));
    delete_id_operation!(delete_rollout, "Delete an ontology rollout.", |id| format!(
        "ontology/rollouts/rollouts/{}",
        encode_path_segment(id)
    ));
    post_id_operation!(start_rollout, "Start an ontology rollout.", |id| format!(
        "ontology/rollouts/rollouts/{}/start",
        encode_path_segment(id)
    ));
    post_id_operation!(pause_rollout, "Pause an ontology rollout.", |id| format!(
        "ontology/rollouts/rollouts/{}/pause",
        encode_path_segment(id)
    ));
    post_id_operation!(resume_rollout, "Resume an ontology rollout.", |id| format!(
        "ontology/rollouts/rollouts/{}/resume",
        encode_path_segment(id)
    ));
    post_id_operation!(
        rollback_rollout,
        "Roll back an ontology rollout.",
        |id| format!(
            "ontology/rollouts/rollouts/{}/rollback",
            encode_path_segment(id)
        )
    );
    get_id_operation!(
        rollout_status,
        "Fetch the status of an ontology rollout.",
        |id| format!(
            "ontology/rollouts/rollouts/{}/status",
            encode_path_segment(id)
        )
    );

    get_query_operation!(
        list_rollups,
        "List ontology rollups.",
        "ontology/rollups/rollups"
    );
    post_operation!(
        create_rollup,
        "Create an ontology rollup.",
        "ontology/rollups/rollups"
    );
    get_id_operation!(get_rollup, "Fetch an ontology rollup.", |id| format!(
        "ontology/rollups/rollups/{}",
        encode_path_segment(id)
    ));
    put_id_operation!(update_rollup, "Update an ontology rollup.", |id| format!(
        "ontology/rollups/rollups/{}",
        encode_path_segment(id)
    ));
    delete_id_operation!(delete_rollup, "Delete an ontology rollup.", |id| format!(
        "ontology/rollups/rollups/{}",
        encode_path_segment(id)
    ));
    post_id_operation!(execute_rollup, "Execute an ontology rollup.", |id| format!(
        "ontology/rollups/rollups/{}/execute",
        encode_path_segment(id)
    ));
    post_id_operation!(preview_rollup, "Preview an ontology rollup.", |id| format!(
        "ontology/rollups/rollups/{}/preview",
        encode_path_segment(id)
    ));
    get_id_operation!(
        rollup_result,
        "Fetch the result of an ontology rollup.",
        |id| format!(
            "ontology/rollups/rollups/{}/result",
            encode_path_segment(id)
        )
    );
    get_id_operation!(
        get_rollup_execution_result,
        "Fetch a rollup execution result.",
        |id| format!(
            "ontology/rollups/rollup-results/{}",
            encode_path_segment(id)
        )
    );

    post_operation!(
        generate_ontology,
        "Generate an ontology from a request.",
        "ontology/engine/ontologies/generate"
    );
    post_operation!(
        validate_ontology,
        "Validate an ontology definition.",
        "ontology/engine/ontologies/validate"
    );
    post_operation!(
        export_ontology,
        "Export an ontology.",
        "ontology/engine/ontologies/export"
    );
    post_operation!(
        export_shacl,
        "Export ontology shapes as SHACL.",
        "ontology/engine/ontologies/export-shacl"
    );
    post_operation!(
        infer_classes,
        "Infer ontology classes from input data.",
        "ontology/engine/ontologies/infer-classes"
    );
    post_operation!(
        infer_properties,
        "Infer ontology properties from input data.",
        "ontology/engine/ontologies/infer-properties"
    );
    post_operation!(
        compare_ontology_versions,
        "Compare ontology versions.",
        "ontology/engine/ontologies/compare-versions"
    );

    post_operation!(
        analyze_extraction,
        "Analyze input for ontology extraction.",
        "ontology/extract/extract/analyze"
    );
    post_operation!(
        extract_architecture,
        "Extract architecture information.",
        "ontology/extract/extract/architecture"
    );
    post_operation!(
        extract_coreferences,
        "Extract coreferences.",
        "ontology/extract/extract/coreferences"
    );
    post_operation!(
        extract_entities,
        "Extract entities.",
        "ontology/extract/extract/entities"
    );
    post_operation!(
        extract_events,
        "Extract events.",
        "ontology/extract/extract/events"
    );
    post_operation!(
        extract_relations,
        "Extract relations.",
        "ontology/extract/extract/relations"
    );
    post_operation!(
        extract_triplets,
        "Extract knowledge triplets.",
        "ontology/extract/extract/triplets"
    );
}
