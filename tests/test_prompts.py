"""Tests for scripts/test_prompts.py functionality."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "scripts"))


class TestLoadPrompts:
    """Tests for prompt loading functionality."""

    def test_load_prompts_from_file(self, temp_json_file):
        """Test loading prompts from a JSON file."""
        from test_prompts import load_prompts

        prompts = load_prompts(temp_json_file)

        assert isinstance(prompts, list)
        assert len(prompts) == 2
        assert prompts[0]["id"] == "test_001"

    def test_load_prompts_structure(self, temp_json_file):
        """Test that loaded prompts have required fields."""
        from test_prompts import load_prompts

        prompts = load_prompts(temp_json_file)

        for prompt in prompts:
            assert "id" in prompt
            assert "category" in prompt
            assert "prompt" in prompt


class TestProjectPaths:
    """Tests for project path configuration."""

    def test_project_root_exists(self, project_root):
        """Verify project root directory exists."""
        assert project_root.exists()
        assert project_root.is_dir()

    def test_scripts_dir_exists(self, project_root):
        """Verify scripts directory exists."""
        scripts_dir = project_root / "scripts"
        assert scripts_dir.exists()

    def test_data_dir_exists(self, data_dir):
        """Verify data directory exists."""
        assert data_dir.exists()


class TestQueryOllama:
    """Tests for Ollama query functionality (mocked)."""

    def test_query_ollama_returns_dict(self, mocker):
        """Test that query_ollama returns expected structure."""
        from test_prompts import query_ollama

        mock_run = mocker.patch("subprocess.run")
        mock_run.return_value = mocker.Mock(
            returncode=0,
            stdout="Test response",
            stderr=""
        )

        result = query_ollama("test prompt", timeout=5)

        assert isinstance(result, dict)
        assert "success" in result
        assert "response" in result
