# Data Processing Pipeline

A comprehensive ETL (Extract, Transform, Load) pipeline built with pandas for data processing and analysis.

## Features

- **Data Loading**: Support for CSV, Excel, JSON, and Parquet formats
- **Data Cleaning**:
  - Duplicate removal
  - Missing value handling
  - Outlier detection and treatment
- **Data Transformation**:
  - Feature engineering
  - Categorical binning
  - Derived metrics
- **Data Analysis**:
  - Summary statistics
  - Group-by aggregations
  - Correlation analysis
  - Performance insights
- **Data Export**: CSV, Excel, and JSON output

## Installation

```bash
pip install -r requirements.txt
```

## Usage

### Basic Usage

```python
from pipeline import DataPipeline

# Run the complete pipeline
pipeline = DataPipeline()
results = pipeline.run_pipeline()
```

### Step-by-Step Usage

```python
from pipeline import DataPipeline

# Initialize pipeline
pipeline = DataPipeline('input_data.csv')

# Run individual steps
pipeline.load_data()
pipeline.clean_data()
pipeline.transform_data()
results = pipeline.analyze_data()
pipeline.export_data('my_output')
```

### Using Your Own Data

```python
pipeline = DataPipeline('path/to/your/data.csv')
results = pipeline.run_pipeline()
```

## Sample Data

If no input file is provided, the pipeline generates sample employee data with:
- 1000 records
- Demographic information (age, name, email)
- Employment details (department, salary, join date)
- Performance metrics
- Intentional missing values and outliers for testing

## Pipeline Stages

### 1. Data Loading
- Automatically detects file format
- Supports multiple file types
- Generates sample data if no file provided

### 2. Data Cleaning
- Removes duplicate records
- Fills missing values:
  - Numeric: median imputation
  - Categorical: mode imputation
- Handles outliers using IQR method

### 3. Data Transformation
Creates new features:
- `years_with_company`: Tenure calculation
- `salary_per_performance`: Efficiency metric
- `age_group`: Categorical age bands
- `performance_percentage`: Normalized score
- `salary_band`: Salary categorization

### 4. Data Analysis
Provides insights on:
- Summary statistics
- Department-level metrics
- Age group analysis
- Salary distribution
- Performance patterns
- Correlation analysis

### 5. Data Export
Exports to:
- CSV: Processed data
- Excel: Formatted workbook
- JSON: Analysis results

## Output Files

The pipeline creates an `output/` directory with:
- `transformed_data.csv`: Cleaned and transformed data
- `transformed_data.xlsx`: Excel version with formatting
- `analysis_results.json`: Comprehensive analysis results

## Analysis Results

The analysis returns a dictionary with:

```python
{
    'summary_statistics': {...},
    'department_analysis': {...},
    'age_analysis': {...},
    'salary_analysis': {...},
    'performance_analysis': {...},
    'salary_correlations': {...}
}
```

## Running the Demo

```bash
python pipeline.py
```

## Running Tests

```bash
pytest test_pipeline.py -v
```

## Example Output

```
============================================================
Starting Data Processing Pipeline
============================================================

No input file provided. Generating sample data...
Loaded 1000 records

Cleaning data...
Initial shape: (1000, 9)
Duplicates removed: 0
Missing values handled: 80 -> 0
Outliers in salary: 42
Final shape: (1000, 9)

Transforming data...
New features created: 5

Analyzing data...

Exporting data to output...
Exported CSV: output/transformed_data.csv
Exported Excel: output/transformed_data.xlsx
Exported JSON: output/analysis_results.json

============================================================
Pipeline completed successfully!
============================================================

--- Summary Statistics ---
total_records: 1000
active_employees: 898
average_age: 48.47
average_salary: 89234.56
average_performance: 3.02

--- Performance Insights ---
High performers: 234 (Avg salary: $91,234.56)
Low performers: 156 (Avg salary: $87,123.45)
```

## Customization

Extend the pipeline by:
1. Adding custom cleaning rules in `clean_data()`
2. Creating new features in `transform_data()`
3. Adding analyses in `analyze_data()`
4. Supporting new export formats in `export_data()`

## Use Cases

- Employee data analysis
- Sales data processing
- Customer analytics
- Financial data transformation
- Marketing metrics analysis
- Any tabular data ETL workflow

## Next Steps

- Add data validation rules
- Implement data quality scoring
- Add visualization generation
- Support database connections
- Add scheduling/automation
- Implement incremental processing
