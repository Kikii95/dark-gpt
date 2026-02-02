"""Tests for scripts/analyze_results.py functionality."""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "scripts"))


class TestLoadResults:
    """Tests for results loading functionality."""

    def test_load_results_from_file(self, tmp_path, sample_results):
        """Test loading results from a JSON file."""
        from analyze_results import load_results

        filepath = tmp_path / "results.json"
        with open(filepath, "w") as f:
            json.dump(sample_results, f)

        results = load_results(filepath)

        assert isinstance(results, dict)
        assert results["model"] == "test-model"
        assert "stats" in results

    def test_load_results_stats_structure(self, tmp_path, sample_results):
        """Test that loaded results have correct stats structure."""
        from analyze_results import load_results

        filepath = tmp_path / "results.json"
        with open(filepath, "w") as f:
            json.dump(sample_results, f)

        results = load_results(filepath)
        stats = results["stats"]

        assert "total" in stats
        assert "success" in stats
        assert "refused" in stats
        assert "error" in stats


class TestCompareModels:
    """Tests for model comparison functionality."""

    def test_compare_models_empty_list(self):
        """Test comparison with empty model list."""
        from analyze_results import compare_models

        result = compare_models([])
        assert result == {}

    def test_compare_models_nonexistent_dirs(self, tmp_path):
        """Test comparison with non-existent directories."""
        from analyze_results import compare_models

        fake_dirs = [
            tmp_path / "nonexistent1",
            tmp_path / "nonexistent2"
        ]

        result = compare_models(fake_dirs)
        assert result == {}


class TestResultsDirectory:
    """Tests for results directory structure."""

    def test_results_dir_exists(self, results_dir):
        """Verify results directory exists."""
        assert results_dir.exists()
        assert results_dir.is_dir()
