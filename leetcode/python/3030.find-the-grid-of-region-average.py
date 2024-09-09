class Solution:
    def resultGrid(self, image: List[List[int]], threshold: int) -> List[List[int]]:
        m, n = len(image), len(image[0])

        sum_matrix = [[0] * n for _ in range(m)]
        count_matrix = [[0] * n for _ in range(m)]

        def is_valid_region(i, j):
            for x in range(i, i + 3):
                for y in range(j, j + 3):
                    if x > i and abs(image[x][y] - image[x - 1][y]) > threshold:
                        return False
                    if y > j and abs(image[x][y] - image[x][y - 1]) > threshold:
                        return False
            return True

        for i in range(m - 2):
            for j in range(n - 2):
                if is_valid_region(i, j):
                    subgrid_sum = 0
                    for x in range(i, i + 3):
                        for y in range(j, j + 3):
                            subgrid_sum += image[x][y]
                    subgrid_sum //= 9
                    for x in range(i, i + 3):
                        for y in range(j, j + 3):
                            sum_matrix[x][y] += subgrid_sum
                            count_matrix[x][y] += 1

        for i in range(m):
            for j in range(n):
                if count_matrix[i][j] > 0:
                    image[i][j] = sum_matrix[i][j] // count_matrix[i][j]

        return image

