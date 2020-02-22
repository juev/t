#include <filesystem>

#include "functions.hpp"
#include "opts/cxxopts.hpp"
#include "sha1/sha1.hpp"

using namespace std;

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
    str = trim(str);
    char hex[SHA1_HEX_SIZE];
    sha1(str.c_str()).finalize().print_hex(hex);
    cout << "sha1('" << str << "'): " << hex << endl;
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