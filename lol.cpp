#include <iostream>
#include <fstream>
#include <clocale>

int main(int argc, char** argv)
{
	setlocale(LC_ALL,"ru_RU.UTF-8");
	std::ifstream input_file(argv[1], std::ifstream::binary);
	if (input_file)
	{
		input_file.seekg(0,input_file.end);
		int length = input_file.tellg();
		input_file.seekg(0,input_file.beg);
		char buffer[length];
		input_file.read(buffer, length);
		input_file.close();
		
	}
	return 0;
}
