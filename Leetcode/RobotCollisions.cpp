// https://leetcode.com/problems/robot-collisions

struct Robot {
    int pos;
    int health;
    char dir;
    int ind;
};

class Solution {
   public:
    vector<int> survivedRobotsHealths(vector<int>& positions,
                                      vector<int>& healths,
                                      string directions) {
        const auto n = positions.size();

        vector<Robot> robots;
        for (int i = 0; i < n; ++i) {
            robots.push_back({positions[i], healths[i], directions[i], i});
        }

        sort(robots.begin(), robots.end(),
             [](const auto& r1, const auto& r2) { return r1.pos < r2.pos; });

        vector<Robot> q;
        for (auto r : robots) {
            if (r.dir == 'R') {
                q.push_back(r);
            } else {
                if (q.empty()) {
                    q.push_back(r);
                } else {
                    bool done = false;

                    while (!q.empty() && !done) {
                        auto last = q.back();
                        q.pop_back();

                        if (last.dir == 'R') {
                            if (last.health < r.health) {
                                r.health -= 1;
                            } else if (last.health > r.health) {
                                last.health -= 1;
                                r.health = 0;
                                q.push_back(last);
                                done = true;
                            } else {
                                r.health = 0;
                                done = true;
                            }
                        } else {
                            q.push_back(last);
                            done = true;
                        }
                    }

                    if (r.health > 0) {
                        q.push_back(r);
                    }
                }
            }
        }

        sort(q.begin(), q.end(),
             [](const auto& r1, const auto& r2) { return r1.ind < r2.ind; });

        vector<int> ans;
        for (int i = 0; i < q.size(); ++i) {
            ans.push_back(q[i].health);
        }

        return ans;
    }
};