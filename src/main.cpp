#include <filesystem>
#include <fstream>
#include <unordered_map>

#include "functions.hpp"
#include "opts/cxxopts.hpp"

namespace fs = std::filesystem;

std::unordered_map<std::string, std::string> tasks = {};
std::unordered_map<std::string, std::string> tasksDone = {};
fs::path taskpath, donepath;
std::string taskdir;
std::string taskfile;

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
    if (tasks.find(prefix) == tasks.end())
      return prefix;
  }
  return hash;
}

void readFiles() {
  std::cout << "taskfile: " << taskpath << std::endl;
  std::ifstream intaskfile(taskpath);

  std::string line;
  while (std::getline(intaskfile, line)) {
    std::istringstream iss(line);
    tasks[sha256_hash(line)] = line;
  }

  std::cout << "done taskfile: " << donepath << std::endl;
  std::ifstream indonefile(donepath);

  while (std::getline(indonefile, line)) {
    std::istringstream iss(line);
    tasksDone[sha256_hash(line)] = line;
  }
}

int main(int argc, char *argv[]) {
  cxxopts::Options options("MyProgram", "One line description of MyProgram");
  options.allow_unrecognised_options().add_options()(
      "positional",
      "Positional arguments: these are the arguments that are entered "
      "without an option",
      cxxopts::value<std::vector<std::string>>())(
      "e,edit", "edit TASK to contain TEXT", cxxopts::value<std::string>())(
      "f,finish", "mark TASK as finished", cxxopts::value<std::string>())(
      "r,remove", "Remove TASK from list", cxxopts::value<std::string>())(
      "l,list", "work on LIST", cxxopts::value<std::string>())(
      "t,taskdir", "work on the lists in DIR", cxxopts::value<std::string>())(
      "d,delete-if-empty", "delete the task file if it becomes empty")(
      "g,grep", "print only tasks that contain WORD",
      cxxopts::value<std::string>())(
      "v,verbose", "print more detailed output (full task ids, etc)")(
      "q,quiet", "print less detailed output (no task ids, etc)")(
      "D,done", "list done tasks instead of unfinished ones")("h,help", "HELP");
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

  std::cout << "Current path is " << taskdir << '\n';

  if (result.count("list")) {
    taskfile = result["list"].as<std::string>();
  } else {
    taskfile = "tasks";
  }

  std::cout << "Current list is " << taskfile << '\n';

  taskpath = fs::path(taskdir) / fs::path(taskfile);
  donepath = fs::path(taskdir) / fs::path("." + taskfile + ".done");

  readFiles();
  for (const auto &n : tasks) {
    std::cout << "Key:[" << n.first << "] Value:[" << n.second << "]\n";
  }
  std::cout << std::endl;
  for (const auto &n : tasksDone) {
    std::cout << "Key:[" << n.first << "] Value:[" << n.second << "]\n";
  }
  std::cout << std::endl;

  if (result.count("positional")) {
    std::string str;
    auto &v = result["positional"].as<std::vector<std::string>>();
    for (const auto &s : v) {
      str += s + " ";
    }
    auto src_str = trim(str);
    // tasks[sha256_hash(str)] = str;
    std::string hash = sha256_hash(str);
    std::cout << "sha256('" << str << "'): " << hash << std::endl;

    std::string p = prefix(hash);
    std::cout << "prefix: " << p << std::endl;
  }
  return 0;
}