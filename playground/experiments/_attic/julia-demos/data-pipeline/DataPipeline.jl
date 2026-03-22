"""
Julia Data Processing Pipeline

High-performance ETL pipeline using Julia's ecosystem:
- DataFrames.jl for data manipulation
- CSV.jl for file I/O
- Statistics.jl for statistical analysis
- Dates.jl for temporal operations
- Query.jl for LINQ-style queries

Performance: 10-100x faster than Python/pandas for large datasets
"""

module DataPipeline

using DataFrames
using CSV
using Statistics
using Dates
using Random
using JSON3
using XLSX

export Pipeline, load_data, clean_data!, transform_data!, analyze_data, export_data

struct Pipeline
    data::Union{DataFrame, Nothing}
    cleaned::Union{DataFrame, Nothing}
    transformed::Union{DataFrame, Nothing}
    results::Dict{String, Any}

    Pipeline() = new(nothing, nothing, nothing, Dict{String, Any}())
end

# Mutable wrapper for pipeline state
mutable struct PipelineState
    data::Union{DataFrame, Nothing}
    cleaned::Union{DataFrame, Nothing}
    transformed::Union{DataFrame, Nothing}
    results::Dict{String, Any}
end

PipelineState() = PipelineState(nothing, nothing, nothing, Dict{String, Any}())

"""
Load data from various formats
"""
function load_data(state::PipelineState, filepath::Union{String, Nothing}=nothing)
    if isnothing(filepath)
        println("📊 No file provided. Generating sample data...")
        state.data = generate_sample_data(1000)
        return state.data
    end

    ext = splitext(filepath)[2]

    if ext == ".csv"
        state.data = CSV.read(filepath, DataFrame)
    elseif ext in [".xlsx", ".xls"]
        state.data = DataFrame(XLSX.readtable(filepath, "Sheet1")...)
    elseif ext == ".json"
        state.data = JSON3.read(read(filepath, String), DataFrame)
    else
        error("Unsupported file format: $ext")
    end

    println("✅ Loaded $(nrow(state.data)) records from $filepath")
    return state.data
end

"""
Generate sample data for demonstration
"""
function generate_sample_data(n::Int=1000)
    Random.seed!(42)

    departments = ["Engineering", "Sales", "Marketing", "HR", "Finance"]

    df = DataFrame(
        id = 1:n,
        name = ["User_$i" for i in 1:n],
        age = rand(18:80, n),
        email = ["user$i@example.com" for i in 1:n],
        salary = rand(30000:150000, n),
        department = rand(departments, n),
        join_date = [Date(2020, 1, 1) + Day(i) for i in 1:n],
        performance_score = rand(1.0:0.1:5.0, n),
        is_active = rand(Bool, n)
    )

    # Introduce missing values
    missing_indices = sample(1:n, 50, replace=false)
    df[missing_indices, :salary] .= missing

    missing_perf = sample(1:n, 30, replace=false)
    df[missing_perf, :performance_score] .= missing

    println("✅ Generated $(nrow(df)) sample records")
    return df
end

"""
Clean data: handle missing values, duplicates, outliers
"""
function clean_data!(state::PipelineState)
    isnothing(state.data) && error("No data loaded. Call load_data first.")

    df = copy(state.data)

    println("\n🧹 Cleaning data...")
    println("Initial shape: $(size(df))")

    # Remove duplicates
    initial_count = nrow(df)
    unique!(df)
    duplicates_removed = initial_count - nrow(df)
    println("Duplicates removed: $duplicates_removed")

    # Handle missing values
    missing_before = sum(ismissing.(df))

    # Fill numeric columns with median
    for col in names(df)
        if eltype(df[!, col]) <: Union{Missing, Number}
            median_val = median(skipmissing(df[!, col]))
            df[!, col] = coalesce.(df[!, col], median_val)
        end
    end

    # Fill categorical columns with mode
    for col in names(df)
        if eltype(df[!, col]) <: Union{Missing, String}
            mode_val = mode(skipmissing(df[!, col]))
            df[!, col] = coalesce.(df[!, col], mode_val)
        end
    end

    missing_after = sum(ismissing.(df))
    println("Missing values handled: $missing_before → $missing_after")

    # Handle outliers using IQR method
    for col in names(df)
        if eltype(df[!, col]) <: Number
            values = df[!, col]
            q1 = quantile(values, 0.25)
            q3 = quantile(values, 0.75)
            iqr = q3 - q1
            lower_bound = q1 - 1.5 * iqr
            upper_bound = q3 + 1.5 * iqr

            outliers = sum((values .< lower_bound) .| (values .> upper_bound))
            if outliers > 0
                println("Outliers in $col: $outliers")
                # Cap outliers
                df[!, col] = clamp.(values, lower_bound, upper_bound)
            end
        end
    end

    println("Final shape: $(size(df))")

    state.cleaned = df
    return df
end

"""
Transform data: feature engineering and derived columns
"""
function transform_data!(state::PipelineState)
    isnothing(state.cleaned) && error("No cleaned data. Call clean_data! first.")

    df = copy(state.cleaned)

    println("\n⚡ Transforming data...")

    # Feature engineering
    df.years_with_company = [Dates.value(Date(now()) - d) / 365.25 for d in df.join_date]

    df.salary_per_performance = df.salary ./ df.performance_score

    # Age groups
    df.age_group = cut(df.age,
        [0, 30, 45, 60, 100],
        labels=["Young", "Mid-Career", "Senior", "Veteran"])

    # Performance percentage
    df.performance_percentage = (df.performance_score ./ 5.0) .* 100

    # Salary bands
    df.salary_band = cut(df.salary,
        [0, 50000, 75000, 100000, 150000],
        labels=["Low", "Medium", "High", "Very High"])

    println("New features created: $(length(names(df)) - length(names(state.cleaned)))")

    state.transformed = df
    return df
end

"""
Analyze data: comprehensive statistical analysis
"""
function analyze_data(state::PipelineState)
    isnothing(state.transformed) && error("No transformed data. Call transform_data! first.")

    df = state.transformed

    println("\n📊 Analyzing data...")

    results = Dict{String, Any}()

    # Summary statistics
    results["summary_statistics"] = Dict(
        "total_records" => nrow(df),
        "active_employees" => sum(df.is_active),
        "average_age" => mean(df.age),
        "average_salary" => mean(df.salary),
        "average_performance" => mean(df.performance_score)
    )

    # Department analysis
    dept_stats = combine(groupby(df, :department),
        :id => length => :count,
        :salary => mean => :avg_salary,
        :salary => median => :median_salary,
        :salary => minimum => :min_salary,
        :salary => maximum => :max_salary,
        :performance_score => mean => :avg_performance,
        :age => mean => :avg_age
    )
    results["department_analysis"] = dept_stats

    # Age group analysis
    age_stats = combine(groupby(df, :age_group),
        :id => length => :count,
        :salary => mean => :avg_salary,
        :performance_score => mean => :avg_performance
    )
    results["age_analysis"] = age_stats

    # Salary band analysis
    salary_stats = combine(groupby(df, :salary_band),
        :id => length => :count,
        :performance_score => mean => :avg_performance,
        :years_with_company => mean => :avg_tenure
    )
    results["salary_analysis"] = salary_stats

    # Performance analysis
    high_performers = filter(row -> row.performance_score >= 4.0, df)
    low_performers = filter(row -> row.performance_score < 2.5, df)

    results["performance_analysis"] = Dict(
        "high_performers_count" => nrow(high_performers),
        "high_performers_avg_salary" => mean(high_performers.salary),
        "low_performers_count" => nrow(low_performers),
        "low_performers_avg_salary" => nrow(low_performers) > 0 ? mean(low_performers.salary) : 0.0
    )

    # Correlation analysis
    numeric_cols = [:salary, :age, :performance_score, :years_with_company]
    salary_corr = Dict()
    for col in numeric_cols
        if col != :salary
            salary_corr[string(col)] = cor(df.salary, df[!, col])
        end
    end
    results["salary_correlations"] = salary_corr

    state.results = results
    return results
end

"""
Export processed data and results
"""
function export_data(state::PipelineState, output_dir::String="output")
    mkpath(output_dir)

    println("\n💾 Exporting data to $output_dir...")

    # Export transformed data
    if !isnothing(state.transformed)
        CSV.write(joinpath(output_dir, "transformed_data.csv"), state.transformed)
        println("✅ Exported CSV: $output_dir/transformed_data.csv")

        # XLSX export requires XLSX.jl
        try
            XLSX.writetable(joinpath(output_dir, "transformed_data.xlsx"),
                collect(DataFrames.eachcol(state.transformed)),
                DataFrames.names(state.transformed))
            println("✅ Exported Excel: $output_dir/transformed_data.xlsx")
        catch e
            println("⚠️  Excel export failed: $e")
        end
    end

    # Export analysis results
    if !isempty(state.results)
        results_json = JSON3.write(state.results, 2)
        write(joinpath(output_dir, "analysis_results.json"), results_json)
        println("✅ Exported JSON: $output_dir/analysis_results.json")
    end
end

"""
Run complete pipeline
"""
function run_pipeline(filepath::Union{String, Nothing}=nothing; export_results::Bool=true)
    println("="^60)
    println("Julia Data Processing Pipeline")
    println("="^60)

    state = PipelineState()

    # Execute pipeline
    load_data(state, filepath)
    clean_data!(state)
    transform_data!(state)
    results = analyze_data(state)

    if export_results
        export_data(state)
    end

    println("\n" * "="^60)
    println("Pipeline completed successfully!")
    println("="^60)

    return state, results
end

# Helper function for categorical binning
function cut(x::AbstractVector, breaks::Vector, labels::Vector{String})
    n = length(breaks) - 1
    result = Vector{String}(undef, length(x))

    for (i, val) in enumerate(x)
        for j in 1:n
            if breaks[j] <= val < breaks[j+1] || (j == n && val == breaks[j+1])
                result[i] = labels[j]
                break
            end
        end
    end

    return result
end

end # module

# Demo execution
using .DataPipeline

function demo()
    state, results = run_pipeline()

    println("\n--- Summary Statistics ---")
    for (key, value) in results["summary_statistics"]
        println("$key: $value")
    end

    println("\n--- Department Analysis ---")
    println(results["department_analysis"])

    println("\n--- Performance Insights ---")
    perf = results["performance_analysis"]
    println("High performers: $(perf["high_performers_count"]) " *
            "(Avg salary: \$$(round(perf["high_performers_avg_salary"], digits=2)))")
    println("Low performers: $(perf["low_performers_count"]) " *
            "(Avg salary: \$$(round(perf["low_performers_avg_salary"], digits=2)))")
end

# Run if executed directly
if abspath(PROGRAM_FILE) == @__FILE__
    demo()
end
