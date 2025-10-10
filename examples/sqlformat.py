import ry

QUERY_MBTILES_DUPLICATE_METADATA_ROWS = """
-- returns the rows that are duplicate (name, value) pairs
select name, value, count(*) as count from metadata group by name having count (*) > 1 and count (distinct value) = 1;
"""

assert (
    ry.sqlfmt(QUERY_MBTILES_DUPLICATE_METADATA_ROWS)
    == """
-- returns the rows that are duplicate (name, value) pairs
select
  name,
  value,
  count(*) as count
from
  metadata
group by
  name
having
  count (*) > 1
  and count (distinct value) = 1;
""".strip()
)


assert (
    ry.sqlfmt(QUERY_MBTILES_DUPLICATE_METADATA_ROWS, uppercase=True)
    == """
-- returns the rows that are duplicate (name, value) pairs
SELECT
  name,
  value,
  count(*) AS count
FROM
  metadata
GROUP BY
  name
HAVING
  count (*) > 1
  AND count (DISTINCT value) = 1;
""".strip()
)
