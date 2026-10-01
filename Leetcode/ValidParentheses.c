// https://leetcode.com/problems/valid-parenthesess

bool isValid(char* s) {
    int j = 0;
    for (int i = 0; i < strlen(s); ++i) {
        if (s[i] == '(' || s[i] == '[' || s[i] == '{')
            s[j++] = s[i];
        else if (s[i] == ')' && j > 0 && s[j - 1] == '(')
            --j;
        else if (s[i] == ']' && j > 0 && s[j - 1] == '[')
            --j;
        else if (s[i] == '}' && j > 0 && s[j - 1] == '{')
            --j;
        else
            return false;
    }
    return j == 0;
}