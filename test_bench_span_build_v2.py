import ry
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

# Span Ranges
#
# | Unit           | Minimum Value                | Maximum Value               |
# | -------------- | ---------------------------- | --------------------------- |
# | `years`        | `-19_998`                    | `19_998`                    |
# | `months`       | `-239_976`                   | `239_976`                   |
# | `weeks`        | `-1_043_497`                 | `1_043_497`                 |
# | `days`         | `-7_304_484`                 | `7_304_484`                 |
# | `hours`        | `-175_307_616`               | `175_307_616`               |
# | `minutes`      | `-10_518_456_960`            | `10_518_456_960`            |
# | `seconds`      | `-631_107_417_600`           | `631_107_417_600`           |
# | `milliseconds` | `-631_107_417_600_000`       | `631_107_417_600_000`       |
# | `microseconds` | `-631_107_417_600_000_000`   | `631_107_417_600_000_000`   |
# | `nanoseconds`  | `-9_223_372_036_854_775_807` | `9_223_372_036_854_775_807` |
import random

_bunch_of_years_to_test = [
    -19_998,
    19_998,
    0,
    1,
    -1,
    *(random.randint(-19_998, 19_998) for _ in range(100)),
]


random_years = lambda: random.randint(-19_998, 19_998)
random_months = lambda: random.randint(-239_976, 239_976)
random_weeks = lambda: random.randint(-1_043_497, 1_043_497)
random_days = lambda: random.randint(-7_304_484, 7_304_484)
random_hours = lambda: random.randint(-175_307_616, 175_307_616)
random_minutes = lambda: random.randint(-10_518_456_960, 10_518_456_960)
random_seconds = lambda: random.randint(-631_107_417_600, 631_107_417_600)
random_milliseconds = lambda: random.randint(-631_107_417_600_000, 631_107_417_600_000)
random_microseconds = lambda: random.randint(
    -631_107_417_600_000_000, 631_107_417_600_000_000
)
random_nanoseconds = lambda: random.randint(
    -9_223_372_036_854_775_807, 9_223_372_036_854_775_807
)
random_bool = lambda: bool(random.getrandbits(1))


def random_timespan_kwargs():
    d = {
        "years": 0,
        "months": 0,
        "weeks": 0,
        "days": 0,
        "hours": 0,
        "minutes": 0,
        "seconds": 0,
        "milliseconds": 0,
        "microseconds": 0,
        "nanoseconds": 0,
    }
    if random_bool():
        d["years"] = random_years()
    if random_bool():
        d["months"] = random_months()
    if random_bool():
        d["weeks"] = random_weeks()
    if random_bool():
        d["days"] = random_days()
    if random_bool():
        d["hours"] = random_hours()
    if random_bool():
        d["minutes"] = random_minutes()
    if random_bool():
        d["seconds"] = random_seconds()
    if random_bool():
        d["milliseconds"] = random_milliseconds()
    if random_bool():
        d["microseconds"] = random_microseconds()
    if random_bool():
        d["nanoseconds"] = random_nanoseconds()
    return d


def generate_1000_random_timespan_kwargs():
    k = []
    for _ in range(100):
        k.append(random_timespan_kwargs())
    return k


_bunch_of_random_timespan_kwargs = generate_1000_random_timespan_kwargs()


def test_benching_release() -> None:
    assert ry.__build_profile__ == "release"


def test_functions_all_equal() -> None:
    fns = [ry.timespan, ry.timespan2, ry.timespan3]
    for kw in _bunch_of_random_timespan_kwargs:
        results = [fn(**kw) for fn in fns]
        assert all(r == results[0] for r in results[1:])


# def test_v1_years(benchmark: "BenchmarkFixture") -> None:
#     benchmark(lambda: ry.timespan(years=10_000))

# def test_v2_years(benchmark: "BenchmarkFixture") -> None:
#     benchmark(lambda: ry.timespan2(years=10_000))


def test_v2_years_array(benchmark: "BenchmarkFixture") -> None:
    @benchmark
    def _run_all2():
        for kw in _bunch_of_random_timespan_kwargs:
            ry.timespan2(**kw)


def test_v3_years_array(benchmark: "BenchmarkFixture") -> None:
    @benchmark
    def _run_all3():
        for kw in _bunch_of_random_timespan_kwargs:
            ry.timespan3(**kw)


def test_v1_years_array(benchmark: "BenchmarkFixture") -> None:
    @benchmark
    def _run_all1():
        for kw in _bunch_of_random_timespan_kwargs:
            ry.timespan(**kw)


# def test_v2_years_array(benchmark: "BenchmarkFixture") -> None:
#     @benchmark
#     def _run_all2():
#         for i in _bunch_of_years_to_test:
#             ry.timespan2(years=i)


# def test_v1_errors(benchmark: "BenchmarkFixture") -> None:
#     @benchmark
#     def _run_all():
#         for years in [-20_000, 20_000]:
#             try:
#                 ry.timespan(years=years)
#             except OverflowError:
#                 pass

# def test_v2_errors(benchmark: "BenchmarkFixture") -> None:
#     @benchmark
#     def _run_all():
#         for years in [-20_000, 20_000]:
#             try:
#                 ry.timespan2(years=years)
#             except ValueError:
#                 pass
