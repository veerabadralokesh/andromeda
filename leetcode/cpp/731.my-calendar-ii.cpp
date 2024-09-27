class MyCalendarTwo {
public:
    MyCalendarTwo() {
        
    }
    
    bool book(int start, int end) {
        cal[start]++;
        cal[end]--;

        int overlaps = 0;
        for (const auto& [_, count]: cal) {
            overlaps += count;
            if (overlaps > 2) {
                if (--cal[start] == 0) {
                    cal.erase(start);
                }
                if (++cal[end] == 0) {
                    cal.erase(end);
                }
                return false;
            }
        }

        return true;
    }
private:
    map<int, int> cal;
};

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * MyCalendarTwo* obj = new MyCalendarTwo();
 * bool param_1 = obj->book(start,end);
 */

