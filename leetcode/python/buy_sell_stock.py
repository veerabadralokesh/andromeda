class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        max_profit = 0
        minp = prices[0]
        maxp = 0
        for price in prices:
            if price < minp:
                minp = price
                maxp = price
            elif price > maxp:
                maxp = price
                if maxp - minp > max_profit:
                    max_profit = maxp - minp
        # print(minp, maxp)
        return max_profit
                

if __name__ == "__main__":
    sol = Solution()
    prices = [7,1,5,3,6,4]
    print(sol.maxProfit(prices))
    prices = [7,6,4,3,1]
    print(sol.maxProfit(prices))
