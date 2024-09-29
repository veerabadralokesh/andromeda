#define ALPHABETS 26

class TrieNode {
   public:
    TrieNode *childNodes[ALPHABETS];
    bool isWordEnd;
    int count;
    TrieNode() {
        isWordEnd = false;
        count = 0;
        for (int i = 0; i < ALPHABETS; i++) {
            childNodes[i] = nullptr;
        }
    }
};

class Trie {
   private:
    TrieNode *root;

   public:
    Trie() {
        root = new TrieNode();
    }

    void insert(const string &key) {
        TrieNode *p = root;
        for (int i = 0; i < key.length(); i++) {
            int j = key[i] - 'a';
            if (!p->childNodes[j]) {
                p->childNodes[j] = new TrieNode();
            }
            p = p->childNodes[j];
            p->count++;
        }
        p->isWordEnd = true;
    }

    int score(const string &key) {
        TrieNode *p = root;
        int score = 0;
        for (int i = 0; i < key.length(); i++) {
            int j = key[i] - 'a';
            if (!p->childNodes[j]) {
                return false;
            }
            p = p->childNodes[j];
            score += p->count;
        }
        return score;
    }

};

class Solution {
public:
    vector<int> sumPrefixScores(vector<string>& words) {
        Trie* t = new Trie();

        for (const auto word: words) {
            t->insert(word);
        }
        
        vector<int> ans(words.size(), 0);
        for (int i=0; i < words.size(); i++) {
            ans[i] = t->score(words[i]);
        }
        return ans;
    }
};

