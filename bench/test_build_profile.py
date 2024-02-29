import warnings

import pytest
from pytest_benchmark.fixture import BenchmarkFixture
import ry


def _warn_benchmarking_with_debug_build() -> None:
    warnings.warn("utiles is built in debug mode", UserWarning, stacklevel=2)


@pytest.mark.filterwarnings("ignore:.*PytestBenchmarkWarning*")
def test_benchmarking_with_debug_build_profile(benchmark: BenchmarkFixture) -> None:
    # warn that this is a debug build
    if not benchmark.disabled and ry.__build_profile__ == "debug":
        _warn_benchmarking_with_debug_build()
    # stupid benchmark to silence pytest-benchmark warning about no benchmarks...
    benchmark(
        lambda: ry.__build_profile__ == "debug" or ry.__build_profile__ == "release",
    )
