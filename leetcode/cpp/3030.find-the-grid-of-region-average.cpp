class Solution {
public:
    vector<vector<int>> resultGrid(vector<vector<int>>& image, int threshold) {
        int m = image.size(), n = image[0].size();

        std::vector<std::vector<int>> sum(m, std::vector<int>(n, 0));
        std::vector<std::vector<int>> counts(m, std::vector<int>(n, 0));

        auto is_valid_region = [&image, threshold](int i, int j) -> bool {
            for (int x = i; x < i + 3; ++x) {
                for (int y = j; y < j + 3; ++y) {
                    if (x > i && std::abs(image[x][y] - image[x - 1][y]) > threshold) {
                        return false;
                    }
                    if (y > j && std::abs(image[x][y] - image[x][y - 1]) > threshold) {
                        return false;
                    }
                }
            }
            return true;
        };

        for (int i = 0; i < m - 2; ++i) {
            for (int j = 0; j < n - 2; ++j) {
                if (is_valid_region(i, j)) {
                    int subgrid_sum = 0;
                    for (int x = i; x < i + 3; ++x) {
                        for (int y = j; y < j + 3; ++y) {
                            subgrid_sum += image[x][y];
                        }
                    }
                    subgrid_sum /= 9;
                    for (int x = i; x < i + 3; ++x) {
                        for (int y = j; y < j + 3; ++y) {
                            sum[x][y] += subgrid_sum;
                            counts[x][y] += 1;
                        }
                    }
                }
            }
        }

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (counts[i][j] > 0) {
                    image[i][j] = sum[i][j] / counts[i][j];
                }
            }
        }

        return image;
    }
};

