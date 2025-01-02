// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/graph_nodes.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A list of nodes in a graph with optional labels, colors, etc.
///
/// ## Example
///
/// ### Simple directed graph
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_graph_directed").spawn()?;
///
///     rec.log(
///         "simple",
///         &[
///             &rerun::GraphNodes::new(["a", "b", "c"])
///                 .with_positions([(0.0, 100.0), (-100.0, 0.0), (100.0, 0.0)])
///                 .with_labels(["A", "B", "C"]) as &dyn rerun::AsComponents,
///             &rerun::GraphEdges::new([("a", "b"), ("b", "c"), ("c", "a")]).with_directed_edges(),
///         ],
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/1200w.png">
///   <img src="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct GraphNodes {
    /// A list of node IDs.
    pub node_ids: Vec<crate::components::GraphNode>,

    /// Optional center positions of the nodes.
    pub positions: Option<Vec<crate::components::Position2D>>,

    /// Optional colors for the boxes.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the node.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<crate::components::ShowLabels>,

    /// Optional radii for nodes.
    pub radii: Option<Vec<crate::components::Radius>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.GraphNode".into(),
            archetype_field_name: Some("node_ids".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphNodes".into()),
            component_name: "rerun.components.GraphNodesIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Position2D".into(),
                archetype_field_name: Some("positions".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Text".into(),
                archetype_field_name: Some("labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.ShowLabels".into(),
                archetype_field_name: Some("show_labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.GraphNode".into(),
                archetype_field_name: Some("node_ids".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.GraphNodesIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Position2D".into(),
                archetype_field_name: Some("positions".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Text".into(),
                archetype_field_name: Some("labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.ShowLabels".into(),
                archetype_field_name: Some("show_labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
        ]
    });

impl GraphNodes {
    /// The total number of components in the archetype: 1 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 7usize;
}

/// Indicator component for the [`GraphNodes`] [`::re_types_core::Archetype`]
pub type GraphNodesIndicator = ::re_types_core::GenericIndicatorComponent<GraphNodes>;

impl ::re_types_core::Archetype for GraphNodes {
    type Indicator = GraphNodesIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.GraphNodes".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Graph nodes"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: GraphNodesIndicator = GraphNodesIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let node_ids = {
            let array = arrays_by_name
                .get("rerun.components.GraphNode")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.GraphNodes#node_ids")?;
            <crate::components::GraphNode>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.GraphNodes#node_ids")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.GraphNodes#node_ids")?
        };
        let positions = if let Some(array) = arrays_by_name.get("rerun.components.Position2D") {
            Some({
                <crate::components::Position2D>::from_arrow2_opt(&**array)
                    .with_context("rerun.archetypes.GraphNodes#positions")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GraphNodes#positions")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow2_opt(&**array)
                    .with_context("rerun.archetypes.GraphNodes#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GraphNodes#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow2_opt(&**array)
                    .with_context("rerun.archetypes.GraphNodes#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GraphNodes#labels")?
            })
        } else {
            None
        };
        let show_labels = if let Some(array) = arrays_by_name.get("rerun.components.ShowLabels") {
            <crate::components::ShowLabels>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.GraphNodes#show_labels")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow2_opt(&**array)
                    .with_context("rerun.archetypes.GraphNodes#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GraphNodes#radii")?
            })
        } else {
            None
        };
        Ok(Self {
            node_ids,
            positions,
            colors,
            labels,
            show_labels,
            radii,
        })
    }
}

impl ::re_types_core::AsComponents for GraphNodes {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.node_ids as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                        archetype_field_name: Some(("node_ids").into()),
                        component_name: ("rerun.components.GraphNode").into(),
                    }),
                }
            }),
            (self
                .positions
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                    archetype_field_name: Some(("positions").into()),
                    component_name: ("rerun.components.Position2D").into(),
                }),
            }),
            (self
                .colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                    archetype_field_name: Some(("colors").into()),
                    component_name: ("rerun.components.Color").into(),
                }),
            }),
            (self
                .labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                    archetype_field_name: Some(("labels").into()),
                    component_name: ("rerun.components.Text").into(),
                }),
            }),
            (self
                .show_labels
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                    archetype_field_name: Some(("show_labels").into()),
                    component_name: ("rerun.components.ShowLabels").into(),
                }),
            }),
            (self
                .radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GraphNodes".into()),
                    archetype_field_name: Some(("radii").into()),
                    component_name: ("rerun.components.Radius").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for GraphNodes {}

impl GraphNodes {
    /// Create a new `GraphNodes`.
    #[inline]
    pub fn new(
        node_ids: impl IntoIterator<Item = impl Into<crate::components::GraphNode>>,
    ) -> Self {
        Self {
            node_ids: node_ids.into_iter().map(Into::into).collect(),
            positions: None,
            colors: None,
            labels: None,
            show_labels: None,
            radii: None,
        }
    }

    /// Optional center positions of the nodes.
    #[inline]
    pub fn with_positions(
        mut self,
        positions: impl IntoIterator<Item = impl Into<crate::components::Position2D>>,
    ) -> Self {
        self.positions = Some(positions.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the boxes.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional text labels for the node.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = Some(show_labels.into());
        self
    }

    /// Optional radii for nodes.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }
}

impl ::re_byte_size::SizeBytes for GraphNodes {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.node_ids.heap_size_bytes()
            + self.positions.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.radii.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::GraphNode>>::is_pod()
            && <Option<Vec<crate::components::Position2D>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<crate::components::ShowLabels>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
    }
}
