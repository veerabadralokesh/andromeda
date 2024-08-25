class Solution:
    def predictPartyVictory(self, senate: str) -> str:
        q = deque(list(senate))
        rr, rd, rcount, dcount = 0, 0, 0, 0
        for s in senate:
            if s == 'R':
                rcount += 1
            else:
                dcount += 1
        while rcount > 0 and dcount > 0:
            s = q.popleft()
            if s == 'R':
                if rr > 0:
                    rr -= 1
                    rcount -= 1
                else:
                    rd += 1
                    q.append('R')
            else:
                if rd > 0:
                    rd -= 1
                    dcount -= 1
                else:
                    rr += 1
                    q.append('D')
        if rcount > 0:
            return "Radiant"
        else:
            return "Dire"

