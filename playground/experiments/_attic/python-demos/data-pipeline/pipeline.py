#!/usr/bin/env python3
"""
Data Processing Pipeline Demo

This module demonstrates a comprehensive data processing pipeline using pandas.
It includes data loading, cleaning, transformation, analysis, and export.
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Optional
import json
from pathlib import Path


class DataPipeline:
    """
    A comprehensive data processing pipeline for ETL operations.
    """

    def __init__(self, input_path: Optional[str] = None):
        """
        Initialize the data pipeline.

        Args:
            input_path: Path to the input data file
        """
        self.input_path = input_path
        self.data: Optional[pd.DataFrame] = None
        self.cleaned_data: Optional[pd.DataFrame] = None
        self.transformed_data: Optional[pd.DataFrame] = None
        self.results: Dict = {}

    def load_data(self, file_path: Optional[str] = None) -> pd.DataFrame:
        """
        Load data from various file formats.

        Args:
            file_path: Path to the data file

        Returns:
            Loaded DataFrame
        """
        path = file_path or self.input_path

        if not path:
            # Generate sample data if no path provided
            print("No input file provided. Generating sample data...")
            self.data = self._generate_sample_data()
            return self.data

        file_extension = Path(path).suffix.lower()

        if file_extension == '.csv':
            self.data = pd.read_csv(path)
        elif file_extension in ['.xlsx', '.xls']:
            self.data = pd.read_excel(path)
        elif file_extension == '.json':
            self.data = pd.read_json(path)
        elif file_extension == '.parquet':
            self.data = pd.read_parquet(path)
        else:
            raise ValueError(f"Unsupported file format: {file_extension}")

        print(f"Loaded {len(self.data)} records from {path}")
        return self.data

    def _generate_sample_data(self, n_records: int = 1000) -> pd.DataFrame:
        """
        Generate sample data for demonstration.

        Args:
            n_records: Number of records to generate

        Returns:
            Sample DataFrame
        """
        np.random.seed(42)

        data = {
            'id': range(1, n_records + 1),
            'name': [f'User_{i}' for i in range(1, n_records + 1)],
            'age': np.random.randint(18, 80, n_records),
            'email': [f'user{i}@example.com' for i in range(1, n_records + 1)],
            'salary': np.random.randint(30000, 150000, n_records),
            'department': np.random.choice(['Engineering', 'Sales', 'Marketing', 'HR', 'Finance'], n_records),
            'join_date': pd.date_range('2020-01-01', periods=n_records, freq='D'),
            'performance_score': np.random.uniform(1.0, 5.0, n_records),
            'is_active': np.random.choice([True, False], n_records, p=[0.9, 0.1])
        }

        # Introduce some missing values and outliers
        df = pd.DataFrame(data)
        df.loc[np.random.choice(df.index, 50, replace=False), 'salary'] = np.nan
        df.loc[np.random.choice(df.index, 30, replace=False), 'performance_score'] = np.nan

        return df

    def clean_data(self) -> pd.DataFrame:
        """
        Clean the data by handling missing values, duplicates, and outliers.

        Returns:
            Cleaned DataFrame
        """
        if self.data is None:
            raise ValueError("No data loaded. Call load_data() first.")

        df = self.data.copy()

        print(f"\nCleaning data...")
        print(f"Initial shape: {df.shape}")

        # Remove duplicates
        initial_count = len(df)
        df = df.drop_duplicates()
        duplicates_removed = initial_count - len(df)
        print(f"Duplicates removed: {duplicates_removed}")

        # Handle missing values
        missing_before = df.isnull().sum().sum()

        # Fill numeric columns with median
        numeric_columns = df.select_dtypes(include=[np.number]).columns
        for col in numeric_columns:
            if df[col].isnull().any():
                df[col].fillna(df[col].median(), inplace=True)

        # Fill categorical columns with mode
        categorical_columns = df.select_dtypes(include=['object']).columns
        for col in categorical_columns:
            if df[col].isnull().any():
                df[col].fillna(df[col].mode()[0], inplace=True)

        missing_after = df.isnull().sum().sum()
        print(f"Missing values handled: {missing_before} -> {missing_after}")

        # Handle outliers using IQR method
        for col in numeric_columns:
            Q1 = df[col].quantile(0.25)
            Q3 = df[col].quantile(0.75)
            IQR = Q3 - Q1
            lower_bound = Q1 - 1.5 * IQR
            upper_bound = Q3 + 1.5 * IQR

            outliers = ((df[col] < lower_bound) | (df[col] > upper_bound)).sum()
            if outliers > 0:
                print(f"Outliers in {col}: {outliers}")
                # Cap outliers instead of removing
                df[col] = df[col].clip(lower_bound, upper_bound)

        print(f"Final shape: {df.shape}")

        self.cleaned_data = df
        return df

    def transform_data(self) -> pd.DataFrame:
        """
        Transform the data with feature engineering and aggregations.

        Returns:
            Transformed DataFrame
        """
        if self.cleaned_data is None:
            raise ValueError("No cleaned data. Call clean_data() first.")

        df = self.cleaned_data.copy()

        print(f"\nTransforming data...")

        # Feature engineering
        df['years_with_company'] = (pd.Timestamp.now() - df['join_date']).dt.days / 365.25
        df['salary_per_performance'] = df['salary'] / df['performance_score']
        df['age_group'] = pd.cut(df['age'], bins=[0, 30, 45, 60, 100],
                                  labels=['Young', 'Mid-Career', 'Senior', 'Veteran'])

        # Normalize performance score to 0-100 scale
        df['performance_percentage'] = (df['performance_score'] / 5.0) * 100

        # Create salary bands
        df['salary_band'] = pd.cut(df['salary'],
                                    bins=[0, 50000, 75000, 100000, 150000],
                                    labels=['Low', 'Medium', 'High', 'Very High'])

        print(f"New features created: {len(df.columns) - len(self.cleaned_data.columns)}")

        self.transformed_data = df
        return df

    def analyze_data(self) -> Dict:
        """
        Perform comprehensive data analysis.

        Returns:
            Dictionary containing analysis results
        """
        if self.transformed_data is None:
            raise ValueError("No transformed data. Call transform_data() first.")

        df = self.transformed_data

        print(f"\nAnalyzing data...")

        results = {
            'summary_statistics': {},
            'department_analysis': {},
            'age_analysis': {},
            'salary_analysis': {},
            'performance_analysis': {}
        }

        # Summary statistics
        results['summary_statistics'] = {
            'total_records': len(df),
            'active_employees': df['is_active'].sum(),
            'average_age': float(df['age'].mean()),
            'average_salary': float(df['salary'].mean()),
            'average_performance': float(df['performance_score'].mean())
        }

        # Department analysis
        dept_stats = df.groupby('department').agg({
            'id': 'count',
            'salary': ['mean', 'median', 'min', 'max'],
            'performance_score': 'mean',
            'age': 'mean'
        }).round(2)

        results['department_analysis'] = dept_stats.to_dict()

        # Age group analysis
        age_stats = df.groupby('age_group').agg({
            'id': 'count',
            'salary': 'mean',
            'performance_score': 'mean'
        }).round(2)

        results['age_analysis'] = age_stats.to_dict()

        # Salary band analysis
        salary_stats = df.groupby('salary_band').agg({
            'id': 'count',
            'performance_score': 'mean',
            'years_with_company': 'mean'
        }).round(2)

        results['salary_analysis'] = salary_stats.to_dict()

        # Performance analysis
        high_performers = df[df['performance_score'] >= 4.0]
        low_performers = df[df['performance_score'] < 2.5]

        results['performance_analysis'] = {
            'high_performers_count': len(high_performers),
            'high_performers_avg_salary': float(high_performers['salary'].mean()),
            'low_performers_count': len(low_performers),
            'low_performers_avg_salary': float(low_performers['salary'].mean())
        }

        # Correlation analysis
        numeric_cols = df.select_dtypes(include=[np.number]).columns
        correlations = df[numeric_cols].corr()['salary'].sort_values(ascending=False)
        results['salary_correlations'] = correlations.to_dict()

        self.results = results
        return results

    def export_data(self, output_dir: str = 'output') -> None:
        """
        Export processed data and analysis results.

        Args:
            output_dir: Directory to save output files
        """
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)

        print(f"\nExporting data to {output_dir}...")

        # Export transformed data
        if self.transformed_data is not None:
            csv_path = output_path / 'transformed_data.csv'
            self.transformed_data.to_csv(csv_path, index=False)
            print(f"Exported CSV: {csv_path}")

            excel_path = output_path / 'transformed_data.xlsx'
            self.transformed_data.to_excel(excel_path, index=False)
            print(f"Exported Excel: {excel_path}")

        # Export analysis results
        if self.results:
            json_path = output_path / 'analysis_results.json'
            with open(json_path, 'w') as f:
                json.dump(self.results, f, indent=2, default=str)
            print(f"Exported JSON: {json_path}")

    def run_pipeline(self, export: bool = True) -> Dict:
        """
        Run the complete data pipeline.

        Args:
            export: Whether to export results

        Returns:
            Analysis results
        """
        print("=" * 60)
        print("Starting Data Processing Pipeline")
        print("=" * 60)

        self.load_data()
        self.clean_data()
        self.transform_data()
        results = self.analyze_data()

        if export:
            self.export_data()

        print("\n" + "=" * 60)
        print("Pipeline completed successfully!")
        print("=" * 60)

        return results


def main():
    """
    Main function to run the pipeline demo.
    """
    pipeline = DataPipeline()
    results = pipeline.run_pipeline()

    print("\n--- Summary Statistics ---")
    for key, value in results['summary_statistics'].items():
        print(f"{key}: {value}")

    print("\n--- Department Analysis ---")
    print(f"Number of departments: {len(results['department_analysis'])}")

    print("\n--- Performance Insights ---")
    perf = results['performance_analysis']
    print(f"High performers: {perf['high_performers_count']} "
          f"(Avg salary: ${perf['high_performers_avg_salary']:,.2f})")
    print(f"Low performers: {perf['low_performers_count']} "
          f"(Avg salary: ${perf['low_performers_avg_salary']:,.2f})")


if __name__ == '__main__':
    main()
