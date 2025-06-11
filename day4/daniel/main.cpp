#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <memory>

int main(int argc, char **argv)
{
    std::fstream file;
    if (argc != 2)
    {
        std::cerr << "Failed to provide file argument" << std::endl;
        return -1;
    }
    file.open(argv[1]);

    std::unique_ptr<std::vector<std::string>> rows_ptr(new std::vector<std::string>);

    std::string line;
    while (getline(file, line))
    {
        rows_ptr->push_back(line);
    }

    auto rows = (*rows_ptr.get());
    int width = rows[0].length();
    int height = rows.size();
    std::string word = "XMAS";

    int count = 0;
    int count_p2 = 0;
    for (int i = 0; i < height; i++)
    {
        for (int j = 0; j < width; j++)
        {
            if (rows[i][j] == word[0])
            {

                // we are on an X, begin search
                int new_count = 8;
                bool dirs[] = {true,
                               true,
                               true,
                               true,
                               true,
                               true,
                               true,
                               true};
                bool SPACE_LEFT = j >= word.length() - 1;
                bool SPACE_RIGHT = j <= width - word.length();
                bool SPACE_UP = i >= word.length() - 1;
                bool SPACE_DOWN = i <= height - word.length();
                int vert_offsets[] = {0, 0, -1, 1, 1, 1, -1, -1};
                int hori_offsets[] = {1, -1, 0, 0, -1, 1, -1, 1};
                bool space_arr[] = {SPACE_RIGHT,
                                    SPACE_LEFT,
                                    SPACE_UP,
                                    SPACE_DOWN,
                                    SPACE_DOWN && SPACE_LEFT,
                                    SPACE_DOWN && SPACE_RIGHT,
                                    SPACE_UP && SPACE_LEFT,
                                    SPACE_UP && SPACE_RIGHT};

                for (int offset = 1; offset < word.length(); offset++)
                {
                    for (int dir = 0; dir < 8; dir++)
                    {
                        if (dirs[dir] && (!space_arr[dir] || rows[i + offset * vert_offsets[dir]][j + offset * hori_offsets[dir]] != word[offset]))
                        {
                            dirs[dir] = false;
                            new_count--;
                        }
                    }
                }
                count += new_count;
            }

            if (rows[i][j] == 'A')
            {
                bool P2_SPACE_LEFT = j >= 1;
                bool P2_SPACE_RIGHT = j < width - 1;
                bool P2_SPACE_UP = i >= 1;
                bool P2_SPACE_DOWN = i < height - 1;

                if (!P2_SPACE_UP || !P2_SPACE_LEFT || !P2_SPACE_DOWN || !P2_SPACE_RIGHT)
                    continue;

                bool TL_IS_M = true;
                bool BL_IS_M = true;

                // for tl to br
                if ((rows[i - 1][j - 1] == 'M' && rows[i + 1][j + 1] == 'S') || (rows[i - 1][j - 1] == 'S' && rows[i + 1][j + 1] == 'M'))
                {
                    // for bl to tr
                    if ((rows[i + 1][j - 1] == 'M' && rows[i - 1][j + 1] == 'S') || (rows[i + 1][j - 1] == 'S' && rows[i - 1][j + 1] == 'M'))
                    {
                        count_p2++;
                    }
                }
            }
        }
    }

    std::cout << count << std::endl;
    std::cout << count_p2 << std::endl;

    return 0;
}