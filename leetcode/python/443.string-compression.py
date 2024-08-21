class Solution:
    def compress(self, chars: List[str]) -> int:
        count = 0
        prev = chars[0]
        i = 0
        for c in chars:
            if c != prev:
                chars[i] = prev
                i += 1
                if count > 1:
                    for rcount in str(count):
                        chars[i] = rcount
                        i += 1
                count = 1
                prev = c
            else:
                count += 1
        chars[i] = prev
        i += 1
        if count > 1:
            for rcount in str(count):
                chars[i] = rcount
                i += 1
        return i

