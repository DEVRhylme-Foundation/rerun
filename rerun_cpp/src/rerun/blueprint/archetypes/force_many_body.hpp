// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_many_body.fbs".

#pragma once

#include "../../blueprint/components/enabled.hpp"
#include "../../blueprint/components/force_strength.hpp"
#include "../../collection.hpp"
#include "../../compiler_utils.hpp"
#include "../../component_batch.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: A force between each pair of nodes that ressembles an electrical charge.
    ///
    /// If `strength` is smaller than 0, it pushes nodes apart, if it is larger than 0 it pulls them together.
    struct ForceManyBody {
        /// Whether the force is enabled.
        std::optional<rerun::blueprint::components::Enabled> enabled;

        /// The strength of the force.
        ///
        /// If `strength` is smaller than 0, it pushes nodes apart, if it is larger than 0 it pulls them together.
        std::optional<rerun::blueprint::components::ForceStrength> strength;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.ForceManyBodyIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        ForceManyBody() = default;
        ForceManyBody(ForceManyBody&& other) = default;

        /// Whether the force is enabled.
        ForceManyBody with_enabled(rerun::blueprint::components::Enabled _enabled) && {
            enabled = std::move(_enabled);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// The strength of the force.
        ///
        /// If `strength` is smaller than 0, it pushes nodes apart, if it is larger than 0 it pulls them together.
        ForceManyBody with_strength(rerun::blueprint::components::ForceStrength _strength) && {
            strength = std::move(_strength);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::ForceManyBody> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const blueprint::archetypes::ForceManyBody& archetype
        );
    };
} // namespace rerun
