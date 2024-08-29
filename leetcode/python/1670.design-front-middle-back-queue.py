class FrontMiddleBackQueue:

    def __init__(self):
        self.frontq = deque()
        self.backq = deque()

    def pushFront(self, val: int) -> None:
        self.frontq.appendleft(val)
        self.moveFrontToBackIfNeeded()
    
    def moveFrontToBackIfNeeded(self):
        if len(self.frontq) == len(self.backq) + 1:
            self.backq.appendleft(self.frontq.pop())

    def pushMiddle(self, val: int) -> None:
        if len(self.frontq) == len(self.backq):
            self.backq.appendleft(val)
        else:
            self.frontq.append(val)
        
    def pushBack(self, val: int) -> None:
        self.backq.append(val)
        self.moveBackToFrontIfNeeded()
    
    def moveBackToFrontIfNeeded(self):
        if len(self.frontq) + 2 == len(self.backq):
            self.frontq.append(self.backq.popleft())
        
    def popFront(self) -> int:
        ret = -1
        if self.frontq:
            ret = self.frontq.popleft()
            self.moveBackToFrontIfNeeded()
        elif self.backq:
            ret = self.backq.popleft()
        return ret
        

    def popMiddle(self) -> int:
        if not self.frontq and not self.backq:
            return -1
        if len(self.frontq) == len(self.backq) - 1:
            return self.backq.popleft()
        return self.frontq.pop()
    


    def popBack(self) -> int:
        # print(self.frontq, self.backq)
        ret = -1
        if self.backq:
            ret = self.backq.pop()
            self.moveFrontToBackIfNeeded()
        return ret


# Your FrontMiddleBackQueue object will be instantiated and called as such:
# obj = FrontMiddleBackQueue()
# obj.pushFront(val)
# obj.pushMiddle(val)
# obj.pushBack(val)
# param_4 = obj.popFront()
# param_5 = obj.popMiddle()
# param_6 = obj.popBack()

