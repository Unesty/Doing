#include <iostream>
#include <fstream>
#include <clocale>
#include <ios>
#include <signal.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>


int main(int argc, char** argv)
{
	char wm[1000000]; //Модель мира
	setlocale(LC_ALL,"ru_RU.UTF-8");
	// Сначала нужно понять как составлять и запускать исполняемый код
	std::ifstream input_file(argv[1], std::ifstream::binary);
	if (input_file)
	{
		input_file.seekg(0,input_file.end);
		int length = input_file.tellg();
		input_file.seekg(0,input_file.beg);
		char* buffer = new char[length];
		input_file.read(buffer, length);
		uint charnumber=0;
		bool exit = 0;
		char slovo[];
		char predlojenie[];
		
		while(exit==0)
		{
			if(charnumber+1>=length)
			{
				charnumber=0;
			}	
			switch(buffer[charnumber])
			{	
				
				//Символы длиной 1 символ. 
				case ' ':
					{
						charnumber++;
						break;
					}
				
				case ',':
					{
						charnumber++;
						
						break;
					}
			}
		}
	        input_file.close();
		
		std::ofstream input_file(argv[1]);
		
		input_file.close();
		
		int size_of_elf;
		char elf[size_of_elf];
		int size_of_path_to_binary;
		char argv[10];
		char path_to_binary[size_of_path_to_binary];
		std::ofstream binary(path_to_binary, std::ios::out | std::ios::binary);
		binary.write(elf,size_of_elf);
		binary.close();
		chmod(path_to_binary, S_IRWXU);
		execv(path_to_binary, NULL);
	}
	
	return 0;
}
