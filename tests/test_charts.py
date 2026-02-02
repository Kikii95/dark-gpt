"""Tests for scripts/generate_charts.py functionality."""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "scripts"))


class TestCheckMatplotlib:
    """Tests for matplotlib availability check."""

    def test_check_matplotlib_installed(self):
        """Test that matplotlib check works when installed."""
        from generate_charts import check_matplotlib

        result = check_matplotlib()
        assert isinstance(result, bool)


class TestLoadLatestResults:
    """Tests for loading latest results functionality."""

    def test_load_latest_results_empty_dir(self, tmp_path):
        """Test loading from directory with no results."""
        from generate_charts import load_latest_results

        result = load_latest_results(tmp_path)
        assert result is None

    def test_load_latest_results_with_file(self, tmp_path, sample_results):
        """Test loading latest results file."""
        from generate_charts import load_latest_results

        results_file = tmp_path / "results_20250202_120000.json"
        with open(results_file, "w") as f:
            json.dump(sample_results, f)

        result = load_latest_results(tmp_path)

        assert result is not None
        assert result["model"] == "test-model"

    def test_load_latest_results_picks_newest(self, tmp_path, sample_results):
        """Test that the newest results file is selected."""
        from generate_charts import load_latest_results
        import time

        old_file = tmp_path / "results_20250201_120000.json"
        with open(old_file, "w") as f:
            json.dump({"model": "old-model", "stats": {}}, f)

        time.sleep(0.1)

        new_file = tmp_path / "results_20250202_120000.json"
        with open(new_file, "w") as f:
            json.dump(sample_results, f)

        result = load_latest_results(tmp_path)
        assert result["model"] == "test-model"


class TestChartsDirectory:
    """Tests for charts directory structure."""

    def test_charts_dir_exists(self, project_root):
        """Verify charts directory exists."""
        charts_dir = project_root / "charts"
        assert charts_dir.exists()
        assert charts_dir.is_dir()
