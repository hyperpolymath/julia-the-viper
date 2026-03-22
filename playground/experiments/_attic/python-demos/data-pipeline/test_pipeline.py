"""
Tests for the data processing pipeline.
"""

import pytest
import pandas as pd
import numpy as np
from pathlib import Path
import json
from pipeline import DataPipeline


class TestDataPipeline:
    """Test suite for DataPipeline class."""

    @pytest.fixture
    def pipeline(self):
        """Create a pipeline instance for testing."""
        return DataPipeline()

    @pytest.fixture
    def sample_data(self):
        """Generate sample data for testing."""
        return pd.DataFrame({
            'id': [1, 2, 3, 4, 5],
            'name': ['Alice', 'Bob', 'Charlie', 'David', 'Eve'],
            'age': [25, 30, 35, 40, 45],
            'salary': [50000, 60000, np.nan, 80000, 90000],
            'department': ['Engineering', 'Sales', 'Engineering', 'HR', 'Sales'],
            'performance_score': [4.5, 3.8, 4.2, np.nan, 4.0]
        })

    def test_initialization(self, pipeline):
        """Test pipeline initialization."""
        assert pipeline.input_path is None
        assert pipeline.data is None
        assert pipeline.cleaned_data is None
        assert pipeline.transformed_data is None
        assert pipeline.results == {}

    def test_generate_sample_data(self, pipeline):
        """Test sample data generation."""
        data = pipeline._generate_sample_data(n_records=100)

        assert isinstance(data, pd.DataFrame)
        assert len(data) == 100
        assert 'id' in data.columns
        assert 'salary' in data.columns
        assert 'department' in data.columns
        assert data.isnull().sum().sum() > 0  # Should have some missing values

    def test_load_data_generates_sample(self, pipeline):
        """Test that load_data generates sample data when no file provided."""
        data = pipeline.load_data()

        assert isinstance(data, pd.DataFrame)
        assert len(data) > 0
        assert pipeline.data is not None

    def test_clean_data_removes_duplicates(self, pipeline):
        """Test duplicate removal in cleaning."""
        # Create data with duplicates
        pipeline.data = pd.DataFrame({
            'id': [1, 1, 2, 3],
            'name': ['Alice', 'Alice', 'Bob', 'Charlie'],
            'value': [10, 10, 20, 30]
        })

        cleaned = pipeline.clean_data()
        assert len(cleaned) == 3  # One duplicate removed

    def test_clean_data_handles_missing_values(self, pipeline, sample_data):
        """Test missing value handling."""
        pipeline.data = sample_data

        # Check initial missing values
        assert pipeline.data['salary'].isnull().sum() > 0
        assert pipeline.data['performance_score'].isnull().sum() > 0

        cleaned = pipeline.clean_data()

        # After cleaning, no missing values
        assert cleaned.isnull().sum().sum() == 0

    def test_transform_data_creates_features(self, pipeline, sample_data):
        """Test feature engineering in transformation."""
        pipeline.data = sample_data
        pipeline.cleaned_data = pipeline.clean_data()

        transformed = pipeline.transform_data()

        # Check new features exist
        assert 'years_with_company' in transformed.columns
        assert 'salary_per_performance' in transformed.columns
        assert 'age_group' in transformed.columns
        assert 'performance_percentage' in transformed.columns
        assert 'salary_band' in transformed.columns

        # Check feature values
        assert (transformed['performance_percentage'] >= 0).all()
        assert (transformed['performance_percentage'] <= 100).all()

    def test_analyze_data_returns_results(self, pipeline):
        """Test data analysis returns proper structure."""
        pipeline.load_data()
        pipeline.clean_data()
        pipeline.transform_data()

        results = pipeline.analyze_data()

        # Check result structure
        assert 'summary_statistics' in results
        assert 'department_analysis' in results
        assert 'age_analysis' in results
        assert 'salary_analysis' in results
        assert 'performance_analysis' in results

        # Check summary statistics
        assert 'total_records' in results['summary_statistics']
        assert 'average_salary' in results['summary_statistics']
        assert 'average_performance' in results['summary_statistics']

    def test_analyze_data_calculates_statistics(self, pipeline):
        """Test that analysis calculates correct statistics."""
        pipeline.load_data()
        pipeline.clean_data()
        pipeline.transform_data()

        results = pipeline.analyze_data()

        stats = results['summary_statistics']
        assert stats['total_records'] > 0
        assert stats['average_age'] > 0
        assert stats['average_salary'] > 0
        assert 0 <= stats['average_performance'] <= 5

    def test_export_data_creates_files(self, pipeline, tmp_path):
        """Test data export creates expected files."""
        pipeline.load_data()
        pipeline.clean_data()
        pipeline.transform_data()
        pipeline.analyze_data()

        output_dir = tmp_path / "test_output"
        pipeline.export_data(str(output_dir))

        # Check files exist
        assert (output_dir / 'transformed_data.csv').exists()
        assert (output_dir / 'transformed_data.xlsx').exists()
        assert (output_dir / 'analysis_results.json').exists()

        # Verify JSON content
        with open(output_dir / 'analysis_results.json') as f:
            json_data = json.load(f)
            assert 'summary_statistics' in json_data

    def test_run_pipeline_executes_all_steps(self, pipeline):
        """Test that run_pipeline executes all steps."""
        results = pipeline.run_pipeline(export=False)

        # Check that all data has been processed
        assert pipeline.data is not None
        assert pipeline.cleaned_data is not None
        assert pipeline.transformed_data is not None
        assert len(pipeline.results) > 0
        assert len(results) > 0

    def test_pipeline_handles_errors_gracefully(self, pipeline):
        """Test error handling in pipeline."""
        # Try to clean without loading
        with pytest.raises(ValueError, match="No data loaded"):
            pipeline.clean_data()

        # Try to transform without cleaning
        with pytest.raises(ValueError, match="No cleaned data"):
            pipeline.transform_data()

        # Try to analyze without transforming
        with pytest.raises(ValueError, match="No transformed data"):
            pipeline.analyze_data()

    def test_department_analysis(self, pipeline):
        """Test department-level analysis."""
        pipeline.load_data()
        pipeline.clean_data()
        pipeline.transform_data()
        results = pipeline.analyze_data()

        dept_analysis = results['department_analysis']
        assert len(dept_analysis) > 0

    def test_performance_analysis(self, pipeline):
        """Test performance analysis."""
        pipeline.load_data()
        pipeline.clean_data()
        pipeline.transform_data()
        results = pipeline.analyze_data()

        perf = results['performance_analysis']
        assert 'high_performers_count' in perf
        assert 'low_performers_count' in perf
        assert 'high_performers_avg_salary' in perf
        assert 'low_performers_avg_salary' in perf

    def test_outlier_handling(self, pipeline):
        """Test outlier detection and handling."""
        # Create data with clear outliers
        pipeline.data = pd.DataFrame({
            'value': [10, 12, 11, 13, 10, 11, 1000]  # 1000 is outlier
        })

        cleaned = pipeline.clean_data()

        # Outlier should be capped
        assert cleaned['value'].max() < 1000

    def test_age_group_creation(self, pipeline, sample_data):
        """Test age group categorization."""
        pipeline.data = sample_data
        pipeline.cleaned_data = pipeline.clean_data()
        transformed = pipeline.transform_data()

        assert 'age_group' in transformed.columns
        assert transformed['age_group'].dtype.name == 'category'

    def test_salary_band_creation(self, pipeline, sample_data):
        """Test salary band categorization."""
        pipeline.data = sample_data
        pipeline.cleaned_data = pipeline.clean_data()
        transformed = pipeline.transform_data()

        assert 'salary_band' in transformed.columns
        assert transformed['salary_band'].dtype.name == 'category'


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
