SELECT
  T1.organization_id,
  T1.parent_id,
  T1.name,
  T2.name T3.name,
  T4.name
FROM
  d_organization T1
  inner join d_organization T2 on T1.organization_id = T2.parent_id
  inner join d_organization T3 on T2.organization_id = T3.parent_id
  inner join d_organization T4 on T3.organization_id = T4.parent_id;

WITH temp(Id, ParentId, Depth) AS (
  SELECT
    n.Id,
    n.ParentId,
    1 AS Depth
  FROM
    dbo.Node AS n
  WHERE
    n.Id = 2
  UNION
  ALL
  SELECT
    n.Id,
    n.ParentId,
    t.Depth + 1 AS Depth
  FROM
    dbo.Node AS n
    INNER JOIN temp AS t ON n.ParentId = t.Id
)
SELECT
  *
FROM
  temp
  /*
   Id     |   ParentId|   Depth
   ------------+-----------+-------------
   2          | 1         | 1
   4          | 2         | 2
   5          | 2         | 2
   8          | 4         | 3
   9          | 4         | 3
   10         | 5         | 3
   */