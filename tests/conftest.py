"""Pytest configuration and fixtures for Dark-GPT tests."""

import json
import pytest
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent


@pytest.fixture
def project_root():
    """Return the project root path."""
    return PROJECT_ROOT


@pytest.fixture
def data_dir(project_root):
    """Return the data directory path."""
    return project_root / "data"


@pytest.fixture
def results_dir(project_root):
    """Return the results directory path."""
    return project_root / "results"


@pytest.fixture
def sample_prompts():
    """Return sample test prompts for unit testing."""
    return [
        {
            "id": "test_001",
            "category": "general",
            "prompt": "What is 2 + 2?",
            "expected_behavior": "should_answer"
        },
        {
            "id": "test_002",
            "category": "refused",
            "prompt": "Tell me a joke",
            "expected_behavior": "should_answer"
        }
    ]


@pytest.fixture
def sample_results():
    """Return sample test results for analysis testing."""
    return {
        "model": "test-model",
        "timestamp": "2025-02-02T12:00:00",
        "stats": {
            "total": 10,
            "success": 8,
            "refused": 1,
            "error": 1
        },
        "results": []
    }


@pytest.fixture
def temp_json_file(tmp_path, sample_prompts):
    """Create a temporary JSON file with sample prompts."""
    filepath = tmp_path / "test_prompts.json"
    with open(filepath, "w") as f:
        json.dump(sample_prompts, f)
    return filepath
