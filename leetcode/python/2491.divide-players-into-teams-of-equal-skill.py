class Solution:
    def dividePlayers(self, skill: List[int]) -> int:
        n = len(skill)//2
        if sum(skill) % n != 0:
            return -1
        teamSkill = sum(skill)//n
        rem = [0] * (1 + teamSkill)
        for s in skill:
            if s >= teamSkill:
                return -1
            rem[s] += 1
        ans = 0
        for i in range(1, 1 + teamSkill//2):
            if rem[i] != rem[teamSkill - i]:
                return -1
            if i != teamSkill - i:
                ans += (i * (teamSkill - i) * rem[i])
            else:
                if rem[i] & 1 == 1:
                    return -1
                ans += (i * (teamSkill - i) * rem[i] // 2)
        return ans

