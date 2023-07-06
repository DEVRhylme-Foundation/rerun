# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)

__all__ = ["AffixFuzzer1"]


@define(str=False, repr=False)
class AffixFuzzer1(Archetype):
    fuzz1001: components.AffixFuzzer1Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer1Array.from_similar,  # type: ignore[misc]
    )
    fuzz1002: components.AffixFuzzer2Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer2Array.from_similar,  # type: ignore[misc]
    )
    fuzz1003: components.AffixFuzzer3Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer3Array.from_similar,  # type: ignore[misc]
    )
    fuzz1004: components.AffixFuzzer4Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer4Array.from_similar,  # type: ignore[misc]
    )
    fuzz1005: components.AffixFuzzer5Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer5Array.from_similar,  # type: ignore[misc]
    )
    fuzz1006: components.AffixFuzzer6Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer6Array.from_similar,  # type: ignore[misc]
    )
    fuzz1007: components.AffixFuzzer7Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer7Array.from_similar,  # type: ignore[misc]
    )
    fuzz1008: components.AffixFuzzer8Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer8Array.from_similar,  # type: ignore[misc]
    )
    fuzz1009: components.AffixFuzzer9Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer9Array.from_similar,  # type: ignore[misc]
    )
    fuzz1010: components.AffixFuzzer10Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer10Array.from_similar,  # type: ignore[misc]
    )
    fuzz1011: components.AffixFuzzer11Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer11Array.from_similar,  # type: ignore[misc]
    )
    fuzz1012: components.AffixFuzzer12Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer12Array.from_similar,  # type: ignore[misc]
    )
    fuzz1013: components.AffixFuzzer13Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer13Array.from_similar,  # type: ignore[misc]
    )
    fuzz1101: components.AffixFuzzer1Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer1Array.from_similar,  # type: ignore[misc]
    )
    fuzz1102: components.AffixFuzzer2Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer2Array.from_similar,  # type: ignore[misc]
    )
    fuzz1103: components.AffixFuzzer3Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer3Array.from_similar,  # type: ignore[misc]
    )
    fuzz1104: components.AffixFuzzer4Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer4Array.from_similar,  # type: ignore[misc]
    )
    fuzz1105: components.AffixFuzzer5Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer5Array.from_similar,  # type: ignore[misc]
    )
    fuzz1106: components.AffixFuzzer6Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer6Array.from_similar,  # type: ignore[misc]
    )
    fuzz1107: components.AffixFuzzer7Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer7Array.from_similar,  # type: ignore[misc]
    )
    fuzz1108: components.AffixFuzzer8Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer8Array.from_similar,  # type: ignore[misc]
    )
    fuzz1109: components.AffixFuzzer9Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer9Array.from_similar,  # type: ignore[misc]
    )
    fuzz1110: components.AffixFuzzer10Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer10Array.from_similar,  # type: ignore[misc]
    )
    fuzz1111: components.AffixFuzzer11Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer11Array.from_similar,  # type: ignore[misc]
    )
    fuzz1112: components.AffixFuzzer12Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer12Array.from_similar,  # type: ignore[misc]
    )
    fuzz1113: components.AffixFuzzer13Array = field(
        metadata={"component": "primary"},
        converter=components.AffixFuzzer13Array.from_similar,  # type: ignore[misc]
    )
    fuzz2001: components.AffixFuzzer1Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer1Array.from_similar,  # type: ignore[misc]
    )
    fuzz2002: components.AffixFuzzer2Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer2Array.from_similar,  # type: ignore[misc]
    )
    fuzz2003: components.AffixFuzzer3Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer3Array.from_similar,  # type: ignore[misc]
    )
    fuzz2004: components.AffixFuzzer4Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer4Array.from_similar,  # type: ignore[misc]
    )
    fuzz2005: components.AffixFuzzer5Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer5Array.from_similar,  # type: ignore[misc]
    )
    fuzz2006: components.AffixFuzzer6Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer6Array.from_similar,  # type: ignore[misc]
    )
    fuzz2007: components.AffixFuzzer7Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer7Array.from_similar,  # type: ignore[misc]
    )
    fuzz2008: components.AffixFuzzer8Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer8Array.from_similar,  # type: ignore[misc]
    )
    fuzz2009: components.AffixFuzzer9Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer9Array.from_similar,  # type: ignore[misc]
    )
    fuzz2010: components.AffixFuzzer10Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer10Array.from_similar,  # type: ignore[misc]
    )
    fuzz2011: components.AffixFuzzer11Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer11Array.from_similar,  # type: ignore[misc]
    )
    fuzz2012: components.AffixFuzzer12Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer12Array.from_similar,  # type: ignore[misc]
    )
    fuzz2013: components.AffixFuzzer13Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer13Array.from_similar,  # type: ignore[misc]
    )
    fuzz2101: components.AffixFuzzer1Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer1Array.from_similar,  # type: ignore[misc]
    )
    fuzz2102: components.AffixFuzzer2Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer2Array.from_similar,  # type: ignore[misc]
    )
    fuzz2103: components.AffixFuzzer3Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer3Array.from_similar,  # type: ignore[misc]
    )
    fuzz2104: components.AffixFuzzer4Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer4Array.from_similar,  # type: ignore[misc]
    )
    fuzz2105: components.AffixFuzzer5Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer5Array.from_similar,  # type: ignore[misc]
    )
    fuzz2106: components.AffixFuzzer6Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer6Array.from_similar,  # type: ignore[misc]
    )
    fuzz2107: components.AffixFuzzer7Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer7Array.from_similar,  # type: ignore[misc]
    )
    fuzz2108: components.AffixFuzzer8Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer8Array.from_similar,  # type: ignore[misc]
    )
    fuzz2109: components.AffixFuzzer9Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer9Array.from_similar,  # type: ignore[misc]
    )
    fuzz2110: components.AffixFuzzer10Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer10Array.from_similar,  # type: ignore[misc]
    )
    fuzz2111: components.AffixFuzzer11Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer11Array.from_similar,  # type: ignore[misc]
    )
    fuzz2112: components.AffixFuzzer12Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer12Array.from_similar,  # type: ignore[misc]
    )
    fuzz2113: components.AffixFuzzer13Array | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.AffixFuzzer13Array.from_similar,  # type: ignore[misc]
    )
    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__