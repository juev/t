#include <filesystem>
#include <unordered_map>

#include "functions.hpp"
#include "opts/cxxopts.hpp"

namespace fs = std::filesystem;

std::unordered_map<std::string, std::string> tasks = {};
std::unordered_map<std::string, std::string> tasksDone = {};
std::unordered_map<std::string, std::string> prefixes = {};

fs::path taskpath, donepath;
std::string taskdir;
std::string taskfile;
bool deleteIfEmpthy;

std::string sha256_hash(std::string text) {
    std::vector<unsigned char> hash(picosha2::k_digest_size);
    picosha2::hash256(text.begin(), text.end(), hash.begin(), hash.end());

    std::string hex_str = picosha2::bytes_to_hex_string(hash.begin(), hash.end());
    return hex_str;
}

std::string prefix(std::string hash) {
    std::string prefix;
    for (size_t i = 1; i <= hash.length(); i++) {
        prefix = hash.substr(0, i);
        if (prefixes.find(prefix) == prefixes.end())
            return prefix;
    }
    return hash;
}

std::string getPrefixByHash(const std::string& hash) {
    for (const auto &n : prefixes) {
        if (n.second == hash)
            return n.first;
    }
    return "";
}

void readFiles() {
    // read task file
    std::ifstream intaskfile(taskpath);

    std::string line;
    while (std::getline(intaskfile, line)) {
        std::istringstream iss(line);
        tasks[sha256_hash(line)] = line;
    }
    intaskfile.close();

    // read done file
    std::ifstream indonefile(donepath);

    while (std::getline(indonefile, line)) {
        std::istringstream iss(line);
        tasksDone[sha256_hash(line)] = line;
    }
    indonefile.close();

    for (const auto &n : tasks) {
        prefixes[prefix(n.first)] = n.first;
    }
}

void writeFiles() {
    if (deleteIfEmpthy && (tasks.empty())) {
        remove(taskpath);
        remove(donepath);
    } else {
        std::ofstream outtaskfile(taskpath);
        if (outtaskfile.is_open()) {
            for (const auto &n : tasks) {
                outtaskfile << n.second << std::endl;
            }
        }
        outtaskfile.close();
        std::ofstream outdonefile(donepath);
        if (outdonefile.is_open()) {
            for (const auto &n : tasksDone) {
                outdonefile << n.second << std::endl;
            }
        }
        outdonefile.close();
    }
}

int main(int argc, char *argv[]) {
    cxxopts::Options options(
            "t", "t is for people that want do things, not organize their tasks.");
    options.allow_unrecognised_options().add_options()(
            "positional",
            "Positional arguments: these are the arguments that are entered "
            "without an option",
            cxxopts::value<std::vector<std::string>>())(
            "e,edit", "edit TASK to contain TEXT", cxxopts::value<std::string>())(
            "f,finish", "mark TASK as finished", cxxopts::value<std::string>())(
            "r,remove", "Remove TASK from list", cxxopts::value<std::string>())(
            "l,list", "work on LIST",
            cxxopts::value<std::string>()->default_value("tasks"))(
            "t,taskdir", "work on the lists in DIR", cxxopts::value<std::string>())(
            "d,delete-if-empty", "delete the task file if it becomes empty",
            cxxopts::value<bool>()->default_value("false"))(
            "g,grep", "print only tasks that contain WORD",
            cxxopts::value<std::string>())(
            "v,verbose", "print more detailed output (full task ids, etc)",
            cxxopts::value<bool>()->default_value("false"))(
            "q,quiet", "print less detailed output (no task ids, etc)",
            cxxopts::value<bool>()->default_value("false"))(
            "done", "list done tasks instead of unfinished ones",
            cxxopts::value<bool>()->default_value("false"))("h,help", "HELP");
    options.parse_positional({"positional"});
    auto result = options.parse(argc, argv);
    const auto &arguments = result.arguments();

    if (result.count("help")) {
        std::cout << options.help() << std::endl;
        exit(0);
    }

    if (result.count("taskdir")) {
        taskdir = result["taskdir"].as<std::string>();
    } else {
        namespace fs = std::filesystem;
        taskdir = fs::current_path();
    }

    if (result.count("list")) {
        taskfile = result["list"].as<std::string>();
    } else {
        taskfile = "tasks";
    }

    taskpath = fs::path(taskdir) / fs::path(taskfile);
    donepath = fs::path(taskdir) / fs::path("." + taskfile + ".done");

    if (!fs::exists(fs::path(taskdir))) {
        std::cout << "Path is not exist: " << fs::path(taskdir) << std::endl;
        exit(1);
    }

    if (result.count("delete-if-empty")) {
        deleteIfEmpthy = true;
    }

    readFiles();

    // edit task
    if (result.count("edit")) {
        auto task_prefix = result["edit"].as<std::string>();
        auto task_hash = prefixes[task_prefix];
        if (tasks.find(task_hash) != tasks.end()) {
            std::string str;
            auto &v = result["positional"].as<std::vector<std::string>>();
            for (const auto &s : v) {
                str += s + " ";
            }
            auto src_str = trim(str);
            if (src_str.length() > 0) {
                tasks[sha256_hash(src_str)] = src_str;
                tasks.erase(task_hash);
                writeFiles();
            }
        } else {
            std::cout << "Task not found: " << task_prefix << std::endl;
        }
        exit(0);
    }

    // finish task
    if (result.count("finish")) {
        auto task_prefix = result["finish"].as<std::string>();
        for (const auto &el : split(task_prefix, ',')) {
            auto task_hash = prefixes[el];
            if (tasks.find(task_hash) != tasks.end()) {
                tasksDone[task_hash] = tasks[task_hash];
                tasks.erase(task_hash);
            } else {
                std::cout << "Task not found: " << el << std::endl;
            }
        }
        writeFiles();
        exit(0);
    }

    // remove task
    if (result.count("remove")) {
        auto task_prefix = result["remove"].as<std::string>();
        for (const auto &el : split(task_prefix, ',')) {
            auto task_hash = prefixes[el];
            if (tasks.find(task_hash) != tasks.end()) {
                tasks.erase(task_hash);
            } else {
                std::cout << "Task not found: " << el << std::endl;
            }
        }
        writeFiles();
        exit(0);
    }

    // add new task
    if (result.count("positional")) {
        std::string str;
        auto &v = result["positional"].as<std::vector<std::string>>();
        for (const auto &s : v) {
            str += s + " ";
        }
        auto src_str = trim(str);
        tasks[sha256_hash(src_str)] = src_str;
        std::string hash = sha256_hash(src_str);
        std::string p = prefix(hash);

        writeFiles();
        exit(0);
    }

    // print tasks
    if (result.count("done")) {
        for (const auto &n : tasksDone) {
            std::cout << n.second << std::endl;
        }
        exit(0);
    }

    std::string word;
    if (result.count("grep")) {
        word = result["grep"].as<std::string>();
    } else {
        word = "";
    }

    for (const auto &n : tasks) {
        if (n.second.find(word) != std::string::npos) {
            if (result.count("quiet")) {
                std::cout << n.second << std::endl;
            } else if (result.count("verbose")) {
                std::cout << n.first << " - " << n.second << std::endl;
            } else {
                std::cout << getPrefixByHash(n.first) << " - " << n.second << std::endl;
            }
        }
    }

    return 0;
}