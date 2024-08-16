class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs[0].length() == 0) {
            return "";
        }
        int i = 0;
        char ref;
        bool done = false;
        while (!done && i < strs[0].length()) {
            ref = strs[0][i];
            for (string s: strs) {
                if (s.length() == i || s[i] != ref) {
                    done = true;
                    break;
                }
            }
            if (!done) {i++;}
        }
        return strs[0].substr(0, i);
    }
};

/*  */

class Solution {
public:
    string longestCommonPrefix(vector<string>& v) {
        string ans="";
        sort(v.begin(),v.end());
        int n=v.size();
        string first=v[0],last=v[n-1];
        for(int i=0;i<min(first.size(),last.size());i++){
            if(first[i]!=last[i]){
                return ans;
            }
            ans+=first[i];
        }
        return ans;
    }
};

