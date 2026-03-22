# SPDX-License-Identifier: PMPL-1.0-or-later

"""Package metadata from registry"""
struct Package
    name::String
    version::String
    description::Union{String,Nothing}
    repository::Union{String,Nothing}
    dependencies::Dict{String,String}
    tarball_url::Union{String,Nothing}
    checksum::Union{String,Nothing}
end

"""Resolved package with all dependencies"""
struct ResolvedPackage
    package::Package
    resolved_deps::Vector{ResolvedPackage}
end
