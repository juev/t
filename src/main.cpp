#include <filesystem>
#include <unordered_map>

#include "functions.hpp"
#include "opts/cxxopts.hpp"

std::unordered_map<std::string, std::string> tasks = {};

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

    // for (const auto &n : tasks) {
    //   std::cout << "Key:[" << n.first << "] Value:[" << n.second << "]\n";
    // }

    std::string p = prefix(hash, tasks);
    std::cout << "prefix: " << p << std::endl;
  }
  std::string taskdir;
  if (result.count("taskdir")) {
    taskdir = result["taskdir"].as<std::string>();
  } else {
    namespace fs = std::filesystem;
    taskdir = fs::current_path();
  }

  std::cout << "Current path is " << taskdir << '\n';
  return 0;
}