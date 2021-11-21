typedef unsigned char   undefined;

typedef unsigned long long    GUID;
typedef unsigned int    ImageBaseOffset32;
typedef unsigned char    bool;
typedef unsigned char    byte;
typedef unsigned int    dword;
float10
typedef long long    longlong;
typedef unsigned char    uchar;
typedef unsigned int    uint;
typedef unsigned long    ulong;
typedef unsigned long long    ulonglong;
typedef unsigned char    undefined1;
typedef unsigned short    undefined2;
typedef unsigned int    undefined4;
typedef unsigned long long    undefined8;
typedef unsigned short    ushort;
typedef short    wchar_t;
typedef unsigned short    word;
#define unkbyte9   unsigned long long
#define unkbyte10   unsigned long long
#define unkbyte11   unsigned long long
#define unkbyte12   unsigned long long
#define unkbyte13   unsigned long long
#define unkbyte14   unsigned long long
#define unkbyte15   unsigned long long
#define unkbyte16   unsigned long long

#define unkuint9   unsigned long long
#define unkuint10   unsigned long long
#define unkuint11   unsigned long long
#define unkuint12   unsigned long long
#define unkuint13   unsigned long long
#define unkuint14   unsigned long long
#define unkuint15   unsigned long long
#define unkuint16   unsigned long long

#define unkint9   long long
#define unkint10   long long
#define unkint11   long long
#define unkint12   long long
#define unkint13   long long
#define unkint14   long long
#define unkint15   long long
#define unkint16   long long

#define unkfloat1   float
#define unkfloat2   float
#define unkfloat3   float
#define unkfloat5   double
#define unkfloat6   double
#define unkfloat7   double
#define unkfloat9   long double
#define unkfloat11   long double
#define unkfloat12   long double
#define unkfloat13   long double
#define unkfloat14   long double
#define unkfloat15   long double
#define unkfloat16   long double

#define BADSPACEBASE   void
#define code   void

typedef union IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion, *PIMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion;

typedef struct IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct, *PIMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct;

struct IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct {
    dword OffsetToDirectory;
    dword DataIsDirectory;
};

union IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion {
    dword OffsetToData;
    struct IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct;
};

typedef unsigned short    wchar16;
typedef void * LPVOID;

typedef LPVOID LPDIRECTDRAW;

typedef struct _cpinfo _cpinfo, *P_cpinfo;

typedef uint UINT;

typedef uchar BYTE;

struct _cpinfo {
    UINT MaxCharSize;
    BYTE DefaultChar[2];
    BYTE LeadByte[12];
};

typedef struct _cpinfo * LPCPINFO;

typedef struct _OVERLAPPED _OVERLAPPED, *P_OVERLAPPED;

typedef ulong ULONG_PTR;

typedef union _union_518 _union_518, *P_union_518;

typedef void * HANDLE;

typedef struct _struct_519 _struct_519, *P_struct_519;

typedef void * PVOID;

typedef ulong DWORD;

struct _struct_519 {
    DWORD Offset;
    DWORD OffsetHigh;
};

union _union_518 {
    struct _struct_519 s;
    PVOID Pointer;
};

struct _OVERLAPPED {
    ULONG_PTR Internal;
    ULONG_PTR InternalHigh;
    union _union_518 u;
    HANDLE hEvent;
};

typedef struct _SYSTEMTIME _SYSTEMTIME, *P_SYSTEMTIME;

typedef ushort WORD;

struct _SYSTEMTIME {
    WORD wYear;
    WORD wMonth;
    WORD wDayOfWeek;
    WORD wDay;
    WORD wHour;
    WORD wMinute;
    WORD wSecond;
    WORD wMilliseconds;
};

typedef struct _TIME_ZONE_INFORMATION _TIME_ZONE_INFORMATION, *P_TIME_ZONE_INFORMATION;

typedef long LONG;

typedef wchar_t WCHAR;

typedef struct _SYSTEMTIME SYSTEMTIME;

struct _TIME_ZONE_INFORMATION {
    LONG Bias;
    WCHAR StandardName[32];
    SYSTEMTIME StandardDate;
    LONG StandardBias;
    WCHAR DaylightName[32];
    SYSTEMTIME DaylightDate;
    LONG DaylightBias;
};

typedef struct _WIN32_FIND_DATAA _WIN32_FIND_DATAA, *P_WIN32_FIND_DATAA;

typedef struct _FILETIME _FILETIME, *P_FILETIME;

typedef struct _FILETIME FILETIME;

typedef char CHAR;

struct _FILETIME {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
};

struct _WIN32_FIND_DATAA {
    DWORD dwFileAttributes;
    FILETIME ftCreationTime;
    FILETIME ftLastAccessTime;
    FILETIME ftLastWriteTime;
    DWORD nFileSizeHigh;
    DWORD nFileSizeLow;
    DWORD dwReserved0;
    DWORD dwReserved1;
    CHAR cFileName[260];
    CHAR cAlternateFileName[14];
};

typedef struct _RTL_CRITICAL_SECTION _RTL_CRITICAL_SECTION, *P_RTL_CRITICAL_SECTION;

typedef struct _RTL_CRITICAL_SECTION * PRTL_CRITICAL_SECTION;

typedef PRTL_CRITICAL_SECTION LPCRITICAL_SECTION;

typedef struct _RTL_CRITICAL_SECTION_DEBUG _RTL_CRITICAL_SECTION_DEBUG, *P_RTL_CRITICAL_SECTION_DEBUG;

typedef struct _RTL_CRITICAL_SECTION_DEBUG * PRTL_CRITICAL_SECTION_DEBUG;

typedef struct _LIST_ENTRY _LIST_ENTRY, *P_LIST_ENTRY;

typedef struct _LIST_ENTRY LIST_ENTRY;

struct _RTL_CRITICAL_SECTION {
    PRTL_CRITICAL_SECTION_DEBUG DebugInfo;
    LONG LockCount;
    LONG RecursionCount;
    HANDLE OwningThread;
    HANDLE LockSemaphore;
    ULONG_PTR SpinCount;
};

struct _LIST_ENTRY {
    struct _LIST_ENTRY * Flink;
    struct _LIST_ENTRY * Blink;
};

struct _RTL_CRITICAL_SECTION_DEBUG {
    WORD Type;
    WORD CreatorBackTraceIndex;
    struct _RTL_CRITICAL_SECTION * CriticalSection;
    LIST_ENTRY ProcessLocksList;
    DWORD EntryCount;
    DWORD ContentionCount;
    DWORD Flags;
    WORD CreatorBackTraceIndexHigh;
    WORD SpareWORD;
};

typedef struct _SECURITY_ATTRIBUTES _SECURITY_ATTRIBUTES, *P_SECURITY_ATTRIBUTES;

typedef struct _SECURITY_ATTRIBUTES * LPSECURITY_ATTRIBUTES;

typedef int BOOL;

struct _SECURITY_ATTRIBUTES {
    DWORD nLength;
    LPVOID lpSecurityDescriptor;
    BOOL bInheritHandle;
};

typedef struct _STARTUPINFOA _STARTUPINFOA, *P_STARTUPINFOA;

typedef CHAR * LPSTR;

typedef BYTE * LPBYTE;

struct _STARTUPINFOA {
    DWORD cb;
    LPSTR lpReserved;
    LPSTR lpDesktop;
    LPSTR lpTitle;
    DWORD dwX;
    DWORD dwY;
    DWORD dwXSize;
    DWORD dwYSize;
    DWORD dwXCountChars;
    DWORD dwYCountChars;
    DWORD dwFillAttribute;
    DWORD dwFlags;
    WORD wShowWindow;
    WORD cbReserved2;
    LPBYTE lpReserved2;
    HANDLE hStdInput;
    HANDLE hStdOutput;
    HANDLE hStdError;
};

typedef struct _STARTUPINFOA STARTUPINFOA;

typedef struct _PROCESS_INFORMATION _PROCESS_INFORMATION, *P_PROCESS_INFORMATION;

typedef struct _PROCESS_INFORMATION * LPPROCESS_INFORMATION;

struct _PROCESS_INFORMATION {
    HANDLE hProcess;
    HANDLE hThread;
    DWORD dwProcessId;
    DWORD dwThreadId;
};

typedef DWORD (* PTHREAD_START_ROUTINE)(LPVOID);

typedef struct _EXCEPTION_POINTERS _EXCEPTION_POINTERS, *P_EXCEPTION_POINTERS;

typedef LONG (* PTOP_LEVEL_EXCEPTION_FILTER)(struct _EXCEPTION_POINTERS *);

typedef PTOP_LEVEL_EXCEPTION_FILTER LPTOP_LEVEL_EXCEPTION_FILTER;

typedef struct _EXCEPTION_RECORD _EXCEPTION_RECORD, *P_EXCEPTION_RECORD;

typedef struct _EXCEPTION_RECORD EXCEPTION_RECORD;

typedef EXCEPTION_RECORD * PEXCEPTION_RECORD;

typedef struct _CONTEXT _CONTEXT, *P_CONTEXT;

typedef struct _CONTEXT CONTEXT;

typedef CONTEXT * PCONTEXT;

typedef struct _FLOATING_SAVE_AREA _FLOATING_SAVE_AREA, *P_FLOATING_SAVE_AREA;

typedef struct _FLOATING_SAVE_AREA FLOATING_SAVE_AREA;

struct _FLOATING_SAVE_AREA {
    DWORD ControlWord;
    DWORD StatusWord;
    DWORD TagWord;
    DWORD ErrorOffset;
    DWORD ErrorSelector;
    DWORD DataOffset;
    DWORD DataSelector;
    BYTE RegisterArea[80];
    DWORD Cr0NpxState;
};

struct _CONTEXT {
    DWORD ContextFlags;
    DWORD Dr0;
    DWORD Dr1;
    DWORD Dr2;
    DWORD Dr3;
    DWORD Dr6;
    DWORD Dr7;
    FLOATING_SAVE_AREA FloatSave;
    DWORD SegGs;
    DWORD SegFs;
    DWORD SegEs;
    DWORD SegDs;
    DWORD Edi;
    DWORD Esi;
    DWORD Ebx;
    DWORD Edx;
    DWORD Ecx;
    DWORD Eax;
    DWORD Ebp;
    DWORD Eip;
    DWORD SegCs;
    DWORD EFlags;
    DWORD Esp;
    DWORD SegSs;
    BYTE ExtendedRegisters[512];
};

struct _EXCEPTION_RECORD {
    DWORD ExceptionCode;
    DWORD ExceptionFlags;
    struct _EXCEPTION_RECORD * ExceptionRecord;
    PVOID ExceptionAddress;
    DWORD NumberParameters;
    ULONG_PTR ExceptionInformation[15];
};

struct _EXCEPTION_POINTERS {
    PEXCEPTION_RECORD ExceptionRecord;
    PCONTEXT ContextRecord;
};

typedef struct _TIME_ZONE_INFORMATION * LPTIME_ZONE_INFORMATION;

typedef PTHREAD_START_ROUTINE LPTHREAD_START_ROUTINE;

typedef struct _OVERLAPPED * LPOVERLAPPED;

typedef struct _WIN32_FIND_DATAA * LPWIN32_FIND_DATAA;

typedef struct _STARTUPINFOA * LPSTARTUPINFOA;

typedef struct _SYSTEMTIME * LPSYSTEMTIME;

typedef DWORD ULONG;

typedef struct _COORD _COORD, *P_COORD;

typedef struct _COORD COORD;

typedef short SHORT;

struct _COORD {
    SHORT X;
    SHORT Y;
};

typedef struct _WINDOW_BUFFER_SIZE_RECORD _WINDOW_BUFFER_SIZE_RECORD, *P_WINDOW_BUFFER_SIZE_RECORD;

struct _WINDOW_BUFFER_SIZE_RECORD {
    COORD dwSize;
};

typedef struct _MOUSE_EVENT_RECORD _MOUSE_EVENT_RECORD, *P_MOUSE_EVENT_RECORD;

struct _MOUSE_EVENT_RECORD {
    COORD dwMousePosition;
    DWORD dwButtonState;
    DWORD dwControlKeyState;
    DWORD dwEventFlags;
};

typedef struct _KEY_EVENT_RECORD _KEY_EVENT_RECORD, *P_KEY_EVENT_RECORD;

typedef struct _KEY_EVENT_RECORD KEY_EVENT_RECORD;

typedef union _union_955 _union_955, *P_union_955;

union _union_955 {
    WCHAR UnicodeChar;
    CHAR AsciiChar;
};

struct _KEY_EVENT_RECORD {
    BOOL bKeyDown;
    WORD wRepeatCount;
    WORD wVirtualKeyCode;
    WORD wVirtualScanCode;
    union _union_955 uChar;
    DWORD dwControlKeyState;
};

typedef struct _MENU_EVENT_RECORD _MENU_EVENT_RECORD, *P_MENU_EVENT_RECORD;

typedef struct _MENU_EVENT_RECORD MENU_EVENT_RECORD;

struct _MENU_EVENT_RECORD {
    UINT dwCommandId;
};

typedef union _union_961 _union_961, *P_union_961;

typedef struct _MOUSE_EVENT_RECORD MOUSE_EVENT_RECORD;

typedef struct _WINDOW_BUFFER_SIZE_RECORD WINDOW_BUFFER_SIZE_RECORD;

typedef struct _FOCUS_EVENT_RECORD _FOCUS_EVENT_RECORD, *P_FOCUS_EVENT_RECORD;

typedef struct _FOCUS_EVENT_RECORD FOCUS_EVENT_RECORD;

struct _FOCUS_EVENT_RECORD {
    BOOL bSetFocus;
};

union _union_961 {
    KEY_EVENT_RECORD KeyEvent;
    MOUSE_EVENT_RECORD MouseEvent;
    WINDOW_BUFFER_SIZE_RECORD WindowBufferSizeEvent;
    MENU_EVENT_RECORD MenuEvent;
    FOCUS_EVENT_RECORD FocusEvent;
};

typedef struct _INPUT_RECORD _INPUT_RECORD, *P_INPUT_RECORD;

struct _INPUT_RECORD {
    WORD EventType;
    union _union_961 Event;
};

typedef struct _INPUT_RECORD * PINPUT_RECORD;

typedef BOOL (* PHANDLER_ROUTINE)(DWORD);

typedef struct tagMSG tagMSG, *PtagMSG;

typedef struct tagMSG MSG;

typedef struct HWND__ HWND__, *PHWND__;

typedef struct HWND__ * HWND;

typedef uint UINT_PTR;

typedef UINT_PTR WPARAM;

typedef long LONG_PTR;

typedef LONG_PTR LPARAM;

typedef struct tagPOINT tagPOINT, *PtagPOINT;

typedef struct tagPOINT POINT;

struct tagPOINT {
    LONG x;
    LONG y;
};

struct tagMSG {
    HWND hwnd;
    UINT message;
    WPARAM wParam;
    LPARAM lParam;
    DWORD time;
    POINT pt;
};

struct HWND__ {
    int unused;
};

typedef struct tagWNDCLASSA tagWNDCLASSA, *PtagWNDCLASSA;

typedef LONG_PTR LRESULT;

typedef LRESULT (* WNDPROC)(HWND, UINT, WPARAM, LPARAM);

typedef struct HINSTANCE__ HINSTANCE__, *PHINSTANCE__;

typedef struct HINSTANCE__ * HINSTANCE;

typedef struct HICON__ HICON__, *PHICON__;

typedef struct HICON__ * HICON;

typedef HICON HCURSOR;

typedef struct HBRUSH__ HBRUSH__, *PHBRUSH__;

typedef struct HBRUSH__ * HBRUSH;

typedef CHAR * LPCSTR;

struct HBRUSH__ {
    int unused;
};

struct tagWNDCLASSA {
    UINT style;
    WNDPROC lpfnWndProc;
    int cbClsExtra;
    int cbWndExtra;
    HINSTANCE hInstance;
    HICON hIcon;
    HCURSOR hCursor;
    HBRUSH hbrBackground;
    LPCSTR lpszMenuName;
    LPCSTR lpszClassName;
};

struct HICON__ {
    int unused;
};

struct HINSTANCE__ {
    int unused;
};

typedef struct tagMSG * LPMSG;

typedef struct tagWNDCLASSA WNDCLASSA;

typedef struct tagMSG * PMSG;

typedef struct tagPALETTEENTRY tagPALETTEENTRY, *PtagPALETTEENTRY;

typedef struct tagPALETTEENTRY PALETTEENTRY;

struct tagPALETTEENTRY {
    BYTE peRed;
    BYTE peGreen;
    BYTE peBlue;
    BYTE peFlags;
};

typedef struct tagPALETTEENTRY * LPPALETTEENTRY;

typedef struct _GUID _GUID, *P_GUID;

struct _GUID {
    ulong Data1;
    ushort Data2;
    ushort Data3;
    uchar Data4[8];
};


// WARNING! conflicting data type names: /guiddef.h/GUID - /GUID

typedef GUID IID;

typedef struct _MEMORY_BASIC_INFORMATION _MEMORY_BASIC_INFORMATION, *P_MEMORY_BASIC_INFORMATION;

typedef ULONG_PTR SIZE_T;

struct _MEMORY_BASIC_INFORMATION {
    PVOID BaseAddress;
    PVOID AllocationBase;
    DWORD AllocationProtect;
    SIZE_T RegionSize;
    DWORD State;
    DWORD Protect;
    DWORD Type;
};

typedef WCHAR * LPWSTR;

typedef CHAR * PCHAR;

typedef WCHAR * LPCWSTR;

typedef long HRESULT;

typedef struct _MEMORY_BASIC_INFORMATION * PMEMORY_BASIC_INFORMATION;

typedef LONG * PLONG;

typedef CHAR * LPCH;

typedef DWORD ACCESS_MASK;

typedef struct IMAGE_DOS_HEADER IMAGE_DOS_HEADER, *PIMAGE_DOS_HEADER;

struct IMAGE_DOS_HEADER {
    char e_magic[2]; // Magic number
    word e_cblp; // Bytes of last page
    word e_cp; // Pages in file
    word e_crlc; // Relocations
    word e_cparhdr; // Size of header in paragraphs
    word e_minalloc; // Minimum extra paragraphs needed
    word e_maxalloc; // Maximum extra paragraphs needed
    word e_ss; // Initial (relative) SS value
    word e_sp; // Initial SP value
    word e_csum; // Checksum
    word e_ip; // Initial IP value
    word e_cs; // Initial (relative) CS value
    word e_lfarlc; // File address of relocation table
    word e_ovno; // Overlay number
    word e_res[4][4]; // Reserved words
    word e_oemid; // OEM identifier (for e_oeminfo)
    word e_oeminfo; // OEM information; e_oemid specific
    word e_res2[10][10]; // Reserved words
    dword e_lfanew; // File address of new exe header
    byte e_program[64]; // Actual DOS program
};

typedef ULONG_PTR DWORD_PTR;

typedef struct tagPOINT * LPPOINT;

typedef struct HKEY__ HKEY__, *PHKEY__;

struct HKEY__ {
    int unused;
};

typedef HINSTANCE HMODULE;

typedef struct tagRECT tagRECT, *PtagRECT;

typedef struct tagRECT RECT;

struct tagRECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
};

typedef int (* FARPROC)(void);

typedef WORD ATOM;

typedef struct tagRECT * LPRECT;

typedef BOOL * LPBOOL;

typedef struct HKEY__ * HKEY;

typedef DWORD * LPDWORD;

typedef DWORD * PDWORD;

typedef struct HDC__ HDC__, *PHDC__;

struct HDC__ {
    int unused;
};

typedef ushort USHORT;

typedef uchar UCHAR;

typedef struct HMENU__ HMENU__, *PHMENU__;

typedef struct HMENU__ * HMENU;

struct HMENU__ {
    int unused;
};

typedef struct _FILETIME * LPFILETIME;

typedef struct HDC__ * HDC;

typedef WORD * LPWORD;

typedef int INT;

typedef HKEY * PHKEY;

typedef void * LPCVOID;

typedef struct IMAGE_OPTIONAL_HEADER32 IMAGE_OPTIONAL_HEADER32, *PIMAGE_OPTIONAL_HEADER32;

typedef struct IMAGE_DATA_DIRECTORY IMAGE_DATA_DIRECTORY, *PIMAGE_DATA_DIRECTORY;

struct IMAGE_DATA_DIRECTORY {
    ImageBaseOffset32 VirtualAddress;
    dword Size;
};

struct IMAGE_OPTIONAL_HEADER32 {
    word Magic;
    byte MajorLinkerVersion;
    byte MinorLinkerVersion;
    dword SizeOfCode;
    dword SizeOfInitializedData;
    dword SizeOfUninitializedData;
    ImageBaseOffset32 AddressOfEntryPoint;
    ImageBaseOffset32 BaseOfCode;
    ImageBaseOffset32 BaseOfData;
    pointer32 ImageBase;
    dword SectionAlignment;
    dword FileAlignment;
    word MajorOperatingSystemVersion;
    word MinorOperatingSystemVersion;
    word MajorImageVersion;
    word MinorImageVersion;
    word MajorSubsystemVersion;
    word MinorSubsystemVersion;
    dword Win32VersionValue;
    dword SizeOfImage;
    dword SizeOfHeaders;
    dword CheckSum;
    word Subsystem;
    word DllCharacteristics;
    dword SizeOfStackReserve;
    dword SizeOfStackCommit;
    dword SizeOfHeapReserve;
    dword SizeOfHeapCommit;
    dword LoaderFlags;
    dword NumberOfRvaAndSizes;
    struct IMAGE_DATA_DIRECTORY DataDirectory[16];
};

typedef struct IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct, *PIMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct;

struct IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct {
    dword NameOffset;
    dword NameIsString;
};

typedef struct IMAGE_FILE_HEADER IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;

struct IMAGE_FILE_HEADER {
    word Machine; // 332
    word NumberOfSections;
    dword TimeDateStamp;
    dword PointerToSymbolTable;
    dword NumberOfSymbols;
    word SizeOfOptionalHeader;
    word Characteristics;
};

typedef struct IMAGE_RESOURCE_DIR_STRING_U_14 IMAGE_RESOURCE_DIR_STRING_U_14, *PIMAGE_RESOURCE_DIR_STRING_U_14;

struct IMAGE_RESOURCE_DIR_STRING_U_14 {
    word Length;
    wchar16 NameString[7];
};

typedef struct IMAGE_NT_HEADERS32 IMAGE_NT_HEADERS32, *PIMAGE_NT_HEADERS32;

struct IMAGE_NT_HEADERS32 {
    char Signature[4];
    struct IMAGE_FILE_HEADER FileHeader;
    struct IMAGE_OPTIONAL_HEADER32 OptionalHeader;
};

typedef union IMAGE_RESOURCE_DIRECTORY_ENTRY IMAGE_RESOURCE_DIRECTORY_ENTRY, *PIMAGE_RESOURCE_DIRECTORY_ENTRY;

typedef union IMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion IMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion, *PIMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion;

union IMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion {
    struct IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct;
    dword Name;
    word Id;
};

union IMAGE_RESOURCE_DIRECTORY_ENTRY {
    union IMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion NameUnion;
    union IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion DirectoryUnion;
};

typedef struct IMAGE_SECTION_HEADER IMAGE_SECTION_HEADER, *PIMAGE_SECTION_HEADER;

typedef union Misc Misc, *PMisc;

typedef enum SectionFlags {
    IMAGE_SCN_ALIGN_1024BYTES=11534336,
    IMAGE_SCN_ALIGN_128BYTES=8388608,
    IMAGE_SCN_ALIGN_16BYTES=5242880,
    IMAGE_SCN_ALIGN_1BYTES=1048576,
    IMAGE_SCN_ALIGN_2048BYTES=12582912,
    IMAGE_SCN_ALIGN_256BYTES=9437184,
    IMAGE_SCN_ALIGN_2BYTES=2097152,
    IMAGE_SCN_ALIGN_32BYTES=6291456,
    IMAGE_SCN_ALIGN_4096BYTES=13631488,
    IMAGE_SCN_ALIGN_4BYTES=3145728,
    IMAGE_SCN_ALIGN_512BYTES=10485760,
    IMAGE_SCN_ALIGN_64BYTES=7340032,
    IMAGE_SCN_ALIGN_8192BYTES=14680064,
    IMAGE_SCN_ALIGN_8BYTES=4194304,
    IMAGE_SCN_CNT_CODE=32,
    IMAGE_SCN_CNT_INITIALIZED_DATA=64,
    IMAGE_SCN_CNT_UNINITIALIZED_DATA=128,
    IMAGE_SCN_GPREL=32768,
    IMAGE_SCN_LNK_COMDAT=4096,
    IMAGE_SCN_LNK_INFO=512,
    IMAGE_SCN_LNK_NRELOC_OVFL=16777216,
    IMAGE_SCN_LNK_OTHER=256,
    IMAGE_SCN_LNK_REMOVE=2048,
    IMAGE_SCN_MEM_16BIT=131072,
    IMAGE_SCN_MEM_DISCARDABLE=33554432,
    IMAGE_SCN_MEM_EXECUTE=536870912,
    IMAGE_SCN_MEM_LOCKED=262144,
    IMAGE_SCN_MEM_NOT_CACHED=67108864,
    IMAGE_SCN_MEM_NOT_PAGED=134217728,
    IMAGE_SCN_MEM_PRELOAD=524288,
    IMAGE_SCN_MEM_PURGEABLE=131072,
    IMAGE_SCN_MEM_READ=1073741824,
    IMAGE_SCN_MEM_SHARED=268435456,
    IMAGE_SCN_MEM_WRITE=2147483648,
    IMAGE_SCN_RESERVED_0001=16,
    IMAGE_SCN_RESERVED_0040=1024,
    IMAGE_SCN_TYPE_NO_PAD=8
} SectionFlags;

union Misc {
    dword PhysicalAddress;
    dword VirtualSize;
};

struct IMAGE_SECTION_HEADER {
    char Name[8];
    union Misc Misc;
    ImageBaseOffset32 VirtualAddress;
    dword SizeOfRawData;
    dword PointerToRawData;
    dword PointerToRelocations;
    dword PointerToLinenumbers;
    word NumberOfRelocations;
    word NumberOfLinenumbers;
    enum SectionFlags Characteristics;
};

typedef struct IMAGE_RESOURCE_DATA_ENTRY IMAGE_RESOURCE_DATA_ENTRY, *PIMAGE_RESOURCE_DATA_ENTRY;

struct IMAGE_RESOURCE_DATA_ENTRY {
    dword OffsetToData;
    dword Size;
    dword CodePage;
    dword Reserved;
};

typedef struct IMAGE_RESOURCE_DIRECTORY IMAGE_RESOURCE_DIRECTORY, *PIMAGE_RESOURCE_DIRECTORY;

struct IMAGE_RESOURCE_DIRECTORY {
    dword Characteristics;
    dword TimeDateStamp;
    word MajorVersion;
    word MinorVersion;
    word NumberOfNamedEntries;
    word NumberOfIdEntries;
};

typedef struct IMAGE_DIRECTORY_ENTRY_EXPORT IMAGE_DIRECTORY_ENTRY_EXPORT, *PIMAGE_DIRECTORY_ENTRY_EXPORT;

struct IMAGE_DIRECTORY_ENTRY_EXPORT {
    dword Characteristics;
    dword TimeDateStamp;
    word MajorVersion;
    word MinorVersion;
    dword Name;
    dword Base;
    dword NumberOfFunctions;
    dword NumberOfNames;
    dword AddressOfFunctions;
    dword AddressOfNames;
    dword AddressOfNameOrdinals;
};

typedef ACCESS_MASK REGSAM;

typedef LONG LSTATUS;

typedef DWORD MCIERROR;

typedef UINT MMRESULT;

typedef UINT MCIDEVICEID;

typedef struct IUnknownVtbl IUnknownVtbl, *PIUnknownVtbl;

typedef struct IUnknown IUnknown, *PIUnknown;

struct IUnknownVtbl {
    HRESULT (* QueryInterface)(struct IUnknown *, IID *, void * *);
    ULONG (* AddRef)(struct IUnknown *);
    ULONG (* Release)(struct IUnknown *);
};

struct IUnknown {
    struct IUnknownVtbl * lpVtbl;
};




void __cdecl FUN_00401010(undefined4 *param_1,undefined4 *param_2,int param_3,uint param_4,uint param_5);
void __cdecl FUN_004010c4(undefined4 *param_1,char *param_2,int param_3,uint param_4,int param_5);
void __cdecl FUN_00401188(undefined4 *param_1,int param_2,int param_3,uint param_4,int param_5,int param_6);
void __cdecl FUN_00401250(undefined4 *param_1,char *param_2,int param_3,uint param_4,int param_5);
void __cdecl FUN_00401320(undefined4 *param_1,int param_2,int param_3,uint param_4,int param_5,int param_6);
undefined4 __cdecl FUN_004013f4(int param_1,undefined4 *param_2);
void __cdecl FUN_0040144c(int param_1,int param_2);
undefined4 __cdecl FUN_004014b8(int param_1,int param_2);
undefined4 __cdecl FUN_00401528(int param_1,int param_2);
undefined4 __cdecl FUN_00401584(int param_1,int param_2);
undefined4 __cdecl FUN_004015f4(int param_1,int param_2);
void __cdecl FUN_00401660(int param_1,int param_2,int param_3,int param_4);
uint __cdecl FUN_00401796(int param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_00401ba7(int param_1,char *param_2);
undefined4 __cdecl FUN_00401ec1(uint **param_1,uint param_2,uint param_3,int *param_4);
uint __cdecl FUN_00402fd5(int param_1,int param_2,char param_3,char param_4);
undefined4 __cdecl FUN_004036ec(int param_1,uint param_2,int param_3);
undefined4 FUN_00403cdc(void);
void __cdecl FUN_00403d5d(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
undefined4 __cdecl FUN_00403dd7(int param_1,int param_2,int param_3);
LPCSTR * __cdecl FUN_00403e94(LPCSTR *param_1,byte param_2);
undefined * FUN_00403f22(void);
undefined4 FUN_00403f41(void);
undefined4 __cdecl FUN_0040425b(uint **param_1,uint param_2,uint param_3,int param_4);
void FUN_00405178(void);
undefined4 __cdecl FUN_00405374(uint **param_1,uint param_2,uint param_3);
void FUN_0040581a(void);
void __cdecl FUN_00405d5a(int param_1);
void FUN_00406851(void);
undefined4 __cdecl FUN_0040690b(uint **param_1,uint param_2,uint param_3);
undefined4 __cdecl FUN_00406cc8(int param_1);
int __cdecl FUN_00406d51(int param_1);
uint FUN_00406eab(void);
undefined4 __cdecl FUN_00406fa2(uint **param_1,uint param_2,int param_3);
int __cdecl FUN_0040705e(int param_1);
int __cdecl FUN_004071ba(int param_1);
int __cdecl FUN_0040721c(int param_1);
void FUN_00407343(void);
undefined4 __cdecl FUN_004074d1(uint **param_1,uint param_2,uint param_3);
uint FUN_00407be4(void);
uint FUN_00407d0e(void);
undefined4 __cdecl FUN_004080a3(int param_1,uint param_2,uint param_3);
void __cdecl FUN_004082e7(int param_1);
void __cdecl FUN_00408371(int param_1);
void __cdecl FUN_004083b2(int param_1);
void __cdecl FUN_0040842c(int param_1);
undefined * FUN_00408447(void);
LPCSTR * __cdecl FUN_00408466(LPCSTR *param_1,byte param_2);
int FUN_004084c9(void);
void __cdecl FUN_004087c0(uint **param_1,int param_2);
undefined4 __cdecl FUN_0040885a(uint **param_1,uint param_2,uint param_3,int *param_4);
void __cdecl FUN_00408d1c(uint **param_1,int param_2,undefined4 param_3);
void __cdecl FUN_00408e35(int *param_1);
void __cdecl FUN_0040910d(int param_1,undefined4 param_2,undefined *param_3);
undefined8 __fastcall FUN_00409207(undefined4 param_1,uint param_2,int param_3,int param_4,uint param_5);
void __cdecl FUN_004093de(uint **param_1);
undefined4 __cdecl FUN_00409581(undefined4 *param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_00409663(int param_1,undefined4 param_2,int param_3,undefined4 param_4,int param_5,int param_6);
int __cdecl FUN_00409fc6(int param_1,int param_2);
void __cdecl FUN_0040a011(int param_1,int param_2);
void __cdecl FUN_0040a121(int param_1,int param_2,int param_3);
uint __cdecl FUN_0040a2cc(int param_1,int param_2);
undefined4 __cdecl FUN_0040a484(uint **param_1,uint param_2,uint param_3,int param_4);
void __cdecl FUN_0040a674(int param_1);
undefined4 __cdecl FUN_0040a70e(int param_1,int param_2,int param_3,int param_4,undefined4 param_5);
void FUN_0040ba36(void);
void FUN_0040babb(void);
DWORD __cdecl FUN_0040bb0a(int param_1,int param_2,DWORD param_3,DWORD param_4);
undefined4 __cdecl FUN_0040bb78(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_0040bc1e(int param_1,int param_2);
int __cdecl FUN_0040bd7e(int param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_0040bf3c(int param_1,int param_2,int param_3,uint param_4);
void __cdecl FUN_0040c15d(int param_1,int param_2,int param_3,undefined4 param_4);
uint FUN_0040c567(void);
undefined4 __cdecl FUN_0040c67d(int param_1,int param_2,int param_3);
int __cdecl sleep_0040c6d8(char *param_1);
void __cdecl FUN_0040c771(char *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7);
void __cdecl FUN_0040c88e(int *param_1);
undefined4 __cdecl FUN_0040c966(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_0040cd10(int param_1);
void FUN_0040d161(void);
void __cdecl FUN_0040d38b(int param_1);
void __cdecl FUN_0040d6e5(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0040d820(undefined4 param_1);
uint FUN_0040d920(void);
undefined4 __cdecl FUN_0040dc25(uint **param_1,uint param_2,int param_3,int param_4);
void __cdecl FUN_0040e175(int param_1,int param_2,int param_3,int param_4);
void FUN_0040e6c5(void);
void FUN_0040e76e(void);
void FUN_0040eba9(void);
void FUN_0040ec2e(void);
void __cdecl FUN_0040f7ac(int param_1,int param_2);
void __cdecl FUN_0040f94b(int param_1,int param_2);
void __cdecl FUN_0040fcb0(int **param_1,uint param_2,uint param_3);
void __cdecl FUN_004100fe(int param_1,uint param_2,uint param_3,int param_4,int param_5);
void FUN_00410497(void);
int FUN_00410758(void);
void FUN_00410846(void);
void __cdecl FUN_004109dc(int param_1,int param_2);
undefined4 __cdecl FUN_00410ae6(int param_1);
void __cdecl FUN_00410b8a(int param_1,int param_2,int param_3);
undefined4 FUN_00410ce7(void);
undefined4 FUN_00410e4d(void);
undefined4 __cdecl FUN_00410fb3(int param_1,uint param_2);
void __cdecl FUN_0041116a(int param_1,int param_2,int param_3);
int __cdecl FUN_00411242(int param_1,int param_2,int param_3);
void __cdecl FUN_004112d1(int param_1,int param_2);
void __cdecl FUN_00411505(int param_1);
undefined4 __cdecl FUN_004115e1(int param_1,int param_2,int param_3,int param_4);
uint __cdecl FUN_004117a9(uint param_1);
undefined4 __cdecl FUN_00411a3d(uint param_1);
void __cdecl FUN_00411b3a(uint **param_1,uint param_2);
undefined4 __cdecl FUN_00411c3b(int param_1);
undefined4 * FUN_00411cab(void);
void FUN_004124b5(void);
void FUN_00412513(void);
void FUN_004125af(void);
void FUN_00412bbf(void);
uint __cdecl FUN_004133bd(uint *param_1,int *param_2);
void __cdecl FUN_00413500(int param_1,char param_2);
void __cdecl FUN_00413546(int param_1);
void __cdecl FUN_00413584(int param_1);
void __cdecl FUN_004135e4(int param_1);
void __cdecl FUN_0041361b(int *param_1);
void __cdecl FUN_00414315(int param_1);
void __cdecl FUN_004144a5(int param_1,int param_2);
void __cdecl FUN_00414592(int param_1,int param_2);
void __cdecl FUN_00414614(uint param_1,int param_2,int param_3,int param_4,int param_5);
void FUN_0041519f(void);
undefined4 * __cdecl FUN_0041529f(int param_1);
void __cdecl FUN_004156e9(int param_1);
void FUN_00415a2e(void);
void FUN_00415a42(void);
undefined4 __cdecl FUN_00416457(int param_1,uint *param_2,uint *param_3);
void FUN_00416cb8(void);
int __cdecl FUN_00416e5d(int param_1);
void FUN_00417035(void);
void FUN_00417257(void);
void FUN_00417354(void);
void FUN_00417a10(void);
undefined4 __cdecl FUN_00417fbe(int *param_1);
void FUN_00418529(void);
int ** FUN_00418819(void);
void __cdecl FUN_00418d7a(int param_1);
undefined4 * __cdecl FUN_00418efd(int param_1,int param_2);
void FUN_00419085(void);
void FUN_00419099(void);
undefined4 * __cdecl FUN_0041a2e3(int param_1);
undefined4 __cdecl FUN_0041a3e0(int param_1,int param_2);
int __cdecl FUN_0041a46a(int param_1,int param_2);
void __cdecl FUN_0041a516(int param_1);
void __cdecl FUN_0041a700(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0041a862(int *param_1,int param_2,int param_3);
int __cdecl FUN_0041ac19(int param_1);
int __cdecl FUN_0041ae21(int param_1,int *param_2,int param_3,int param_4,undefined4 *param_5,undefined4 *param_6);
undefined4 __cdecl FUN_0041b05a(int param_1,int param_2,uint param_3);
undefined4 * __cdecl FUN_0041ba73(undefined4 *param_1);
void __cdecl FUN_0041bbec(undefined4 *param_1);
undefined4 __cdecl FUN_0041be7a(int param_1);
void __cdecl FUN_0041c01d(int param_1);
void __cdecl FUN_0041c3cf(int param_1,int param_2);
void __cdecl FUN_0041c67a(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0041c726(int param_1);
undefined4 __cdecl FUN_0041c9b2(int param_1,int param_2);
void FUN_0041cad6(void);
undefined4 __cdecl FUN_0041cffc(int param_1,int param_2);
int FUN_0041d054(void);
void FUN_0041d3ba(void);
void FUN_0041d539(void);
undefined4 __cdecl FUN_0041d5ed(int param_1,int param_2,int param_3,uint param_4,uint *param_5,uint *param_6);
void FUN_0041d83c(void);
void FUN_0041d87d(void);
void FUN_0041da19(void);
undefined4 FUN_0041dbe9(void);
int __cdecl FUN_0041dc37(int param_1,int param_2,int param_3,uint param_4);
undefined4 * __cdecl FUN_0041dd5a(int param_1,int param_2,int param_3);
void FUN_0041de20(void);
void FUN_0041de42(void);
int __cdecl FUN_0041e418(int param_1,int param_2);
int __cdecl FUN_0041e4f1(int param_1,int param_2);
undefined4 __cdecl FUN_0041ec1d(int param_1,uint param_2);
undefined4 __cdecl FUN_0041ed44(int param_1,uint param_2);
void __cdecl FUN_0041f4d4(int param_1,int param_2,int param_3);
void __cdecl FUN_0041f524(int param_1,uint param_2,int param_3,int param_4,undefined2 param_5);
void __cdecl FUN_0041f600(undefined4 param_1,uint param_2,int param_3);
int __cdecl FUN_0041f6d8(undefined4 param_1,uint param_2,int param_3,int param_4);
bool __cdecl FUN_0041f754(undefined4 param_1,uint param_2,int param_3,int param_4,undefined4 *param_5,undefined4 *param_6,undefined4 *param_7);
undefined4 __cdecl FUN_0041fa61(int param_1,undefined param_2);
void __cdecl FUN_00420142(undefined4 param_1,uint param_2,int param_3);
void __cdecl FUN_004201f5(undefined4 param_1,int param_2,int param_3);
void __cdecl FUN_0042023b(undefined4 param_1,int param_2,int param_3);
void __cdecl FUN_004202bf(undefined4 param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00420343(int param_1,undefined param_2);
void __cdecl FUN_004206c7(undefined4 param_1,int param_2,int param_3);
undefined4 FUN_0042074b(void);
void __cdecl FUN_00420770(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0042082d(int param_1);
undefined4 __cdecl FUN_00420c5e(undefined4 param_1);
void __cdecl FUN_00420c87(undefined4 param_1);
void __cdecl FUN_00420cb1(int param_1,undefined param_2,undefined param_3,undefined param_4);
void __cdecl FUN_00420d18(int param_1,uint param_2,uint param_3,uint param_4);
undefined4 __cdecl FUN_00420dbb(int param_1,uint param_2,uint param_3,uint param_4);
void FUN_00420e3e(void);
void __cdecl FUN_00420edd(int param_1);
void FUN_00420fd9(void);
int __cdecl FUN_00421079(int param_1);
int __cdecl FUN_004210ba(int param_1,int param_2,int param_3);
int __cdecl FUN_0042126f(int param_1,int param_2);
void FUN_0042145e(void);
undefined4 __cdecl FUN_0042285f(int param_1,int param_2);
void FUN_004228d7(void);
uint __cdecl FUN_00423530(int param_1);
int __cdecl FUN_00423615(int param_1,int param_2,int param_3);
uint __cdecl FUN_0042376e(int param_1,int param_2);
undefined4 * __cdecl FUN_004238dc(int param_1);
void __cdecl FUN_00423961(int param_1);
void __cdecl FUN_004239de(int param_1);
undefined4 * FUN_00423ac1(void);
void __cdecl FUN_00423cfb(int param_1);
void FUN_00423df5(void);
void FUN_00423f15(void);
void FUN_0042433b(void);
undefined4 __cdecl FUN_0042444e(int param_1,uint param_2,undefined4 param_3,undefined4 param_4);
void __cdecl FUN_004245ce(int param_1);
void FUN_004248db(void);
void __cdecl FUN_00424d33(int param_1);
void __cdecl FUN_00424e53(int param_1,int param_2);
void FUN_00424fe4(void);
int __cdecl FUN_0042530b(uint param_1);
void FUN_00425463(void);
int __cdecl FUN_004255c0(int param_1);
undefined4 * FUN_004257de(void);
void __cdecl FUN_00425e18(int param_1,int *param_2,int *param_3);
uint __cdecl FUN_00425fb6(int param_1,uint *param_2,uint *param_3);
void __cdecl FUN_0042629a(int param_1);
undefined4 __cdecl FUN_00426421(int param_1,uint param_2,int param_3,int param_4);
uint __cdecl FUN_0042668f(int param_1,uint param_2,uint param_3);
int __cdecl FUN_004266f4(ushort param_1);
void FUN_00426782(void);
void FUN_00426a99(void);
void FUN_00426bcf(void);
void FUN_00426c24(void);
undefined4 __cdecl FUN_00426d07(int param_1,uint param_2,int param_3);
void FUN_00426edc(void);
undefined4 __cdecl FUN_004271e8(uint **param_1,uint param_2,undefined4 param_3,int param_4);
void FUN_0042732a(void);
undefined4 __cdecl FUN_0042769d(uint **param_1,uint param_2,uint param_3);
int FUN_00427bc4(void);
undefined4 __cdecl FUN_00427ffc(uint **param_1,uint param_2,uint param_3);
void FUN_004281cd(void);
void __cdecl FUN_00428288(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00428bd5(int param_1);
undefined4 __cdecl FUN_00428d05(uint **param_1,uint param_2,uint param_3);
uint __cdecl FUN_00429b8c(int param_1);
uint FUN_0042ae4b(void);
undefined4 __cdecl FUN_0042af6f(uint **param_1,undefined4 param_2,uint param_3);
uint FUN_0042b3d1(void);
int FUN_0042b505(void);
undefined4 __cdecl FUN_0042b67e(uint **param_1,uint param_2,uint param_3);
uint __cdecl FUN_0042b826(undefined4 param_1);
undefined4 __cdecl FUN_0042b9c0(int param_1,uint param_2,int param_3,int param_4);
uint FUN_0042bf4e(void);
undefined4 __cdecl FUN_0042c3e9(int param_1,uint param_2,uint param_3);
uint FUN_0042c5a2(void);
undefined4 __cdecl FUN_0042c699(int param_1,int param_2,uint param_3);
void __cdecl FUN_0042c77c(uint *param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_0042ce75(int param_1,int param_2);
void __cdecl FUN_0042d03d(int param_1,int param_2);
int __cdecl FUN_0042d188(int param_1,int param_2);
int __cdecl FUN_0042d295(int param_1);
int __cdecl FUN_0042d379(int param_1,int param_2);
void FUN_0042d418(void);
int __cdecl FUN_0042d4ca(int param_1,int param_2);
DWORD __cdecl check_reg_val_0042d54b(LPBYTE byte_pointer);
undefined4 __cdecl FUN_0042d5cf(LPBYTE param_1);
dword important_func_0042d653(HINSTANCE param_1,DWORD param_2,undefined4 param_3,DWORD param_4);
void FUN_0042e42d(void);
void FUN_0042e583(void);
void __cdecl FUN_0042e642(uint **param_1);
void __cdecl FUN_0042e670(int param_1,uint **param_2);
void __cdecl FUN_0042e871(uint **param_1);
int FUN_0042f693(void);
int FUN_0042f6fe(void);
void __cdecl FUN_0042f769(int param_1);
void FUN_0042f801(void);
undefined4 FUN_0042f8ac(void);
int __cdecl FUN_0042f963(int param_1,int param_2,int param_3);
void FUN_0042fa19(void);
void __cdecl FUN_0042fcf9(int *param_1,int param_2);
void __cdecl FUN_0042fd26(int *param_1,int param_2);
int __cdecl FUN_0042fd53(int param_1);
void FUN_0042fdcd(void);
undefined4 __cdecl FUN_0042fe63(int param_1,int param_2,int param_3);
void FUN_0042feac(void);
void __cdecl FUN_0042fefc(int param_1,int *param_2,char *param_3,int param_4,int param_5);
void __cdecl FUN_00430418(int param_1,int *param_2,char *param_3);
undefined4 __cdecl FUN_00430444(uint **param_1,uint param_2,int param_3,int param_4);
void FUN_00430604(void);
int __cdecl FUN_00430815(int param_1);
void FUN_00430868(void);
void FUN_00430af4(void);
int __cdecl FUN_00430bce(int param_1);
char * __cdecl FUN_00430c19(char *param_1,undefined4 param_2);
undefined4 __cdecl FUN_00430d15(char *param_1);
BOOL __cdecl FUN_00430d8a(LPCSTR app_name,LPCSTR key_name,DWORD param_3,LPCSTR file_name);
undefined4 * __cdecl FUN_00430dd1(undefined4 *param_1);
LPCSTR * __cdecl FUN_00430df7(LPCSTR *param_1);
void __cdecl FUN_00430e38(LPCSTR *param_1);
void __cdecl FUN_00430e6a(LPCSTR *param_1);
int __cdecl FUN_00430e9c(int param_1);
undefined4 __cdecl FUN_00430ef1(int param_1);
undefined4 * __cdecl FUN_00430f3a(undefined4 *param_1);
LPCSTR * __cdecl FUN_00430f78(LPCSTR *param_1);
int __cdecl FUN_00430fd2(int param_1);
undefined4 __cdecl FUN_00431006(undefined4 param_1);
undefined4 __cdecl FUN_00431034(undefined4 param_1);
undefined4 __cdecl FUN_00431062(undefined4 param_1);
undefined4 __cdecl FUN_00431090(undefined4 param_1);
undefined4 __cdecl FUN_004310be(undefined4 param_1);
undefined4 __cdecl FUN_004310ec(undefined4 param_1);
undefined4 __cdecl FUN_0043111a(undefined4 param_1);
int FUN_00431148(void);
int __cdecl FUN_00431230(int param_1,int param_2,int param_3);
void __cdecl FUN_004312f0(int param_1,int param_2,int param_3,int param_4,int param_5);
undefined4 FUN_00431980(void);
undefined4 __cdecl FUN_00431a7b(uint **param_1,undefined4 param_2,int param_3);
void FUN_00431c53(void);
void __cdecl FUN_00431d0a(undefined4 *param_1);
undefined4 * __cdecl FUN_00431d31(undefined4 *param_1);
void __cdecl FUN_00431d5a(undefined4 *param_1,undefined4 *param_2);
void __cdecl FUN_00431dec(int param_1,int param_2);
void __cdecl FUN_00431efd(int param_1,int param_2);
void __cdecl FUN_00431f8d(undefined4 *param_1,int param_2);
void __cdecl FUN_0043234c(int param_1,int param_2);
void __cdecl FUN_00432515(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
void __cdecl FUN_00432683(undefined4 *param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_0043284d(int param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_00432a04(int param_1,undefined param_2,undefined param_3);
void __cdecl FUN_00432a46(int param_1);
void __cdecl FUN_00432a84(int param_1,int param_2);
int __cdecl FUN_00432aca(int param_1,int param_2);
int __cdecl FUN_00432b1a(int param_1);
int __cdecl FUN_00432bd3(int param_1);
int __cdecl FUN_00432c94(int param_1);
undefined4 __cdecl FUN_00432cec(int param_1);
undefined4 __cdecl FUN_00432d43(int param_1,int param_2);
void __cdecl FUN_00432de2(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00432e4c(int param_1);
undefined4 __cdecl FUN_00432ea6(int param_1);
undefined4 __cdecl FUN_00432f12(int param_1);
undefined4 __cdecl FUN_00432f5e(int param_1);
bool __cdecl FUN_00432fca(int param_1,int param_2);
undefined4 __cdecl FUN_00433083(int param_1,int param_2);
undefined4 __cdecl FUN_004330dc(int param_1,int param_2);
void __cdecl FUN_0043322a(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_00433545(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00433916(int param_1,int param_2,int param_3);
void __cdecl FUN_0043402e(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00434129(uint *param_1,int param_2,int param_3);
void __cdecl FUN_004348e8(int param_1,int param_2,int param_3);
int __cdecl FUN_00434baf(int param_1);
undefined4 __cdecl FUN_00434bfa(int param_1);
undefined4 __cdecl FUN_00434c46(int param_1,int param_2);
undefined4 __cdecl FUN_00434cb1(int param_1);
undefined4 __cdecl FUN_00434d1f(int param_1);
int __cdecl FUN_00434de1(int param_1);
undefined4 __cdecl FUN_00434e1a(int param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_00434f84(int param_1);
void __cdecl FUN_00435045(int param_1);
void __cdecl FUN_0043507c(int param_1);
undefined4 __cdecl FUN_004350b3(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00435263(int param_1,int param_2);
void __cdecl FUN_00435409(int param_1);
undefined4 __cdecl FUN_00435440(int param_1);
undefined4 __cdecl FUN_0043548c(int param_1);
undefined4 __cdecl FUN_004354d8(int param_1);
void __cdecl FUN_00435524(undefined4 param_1,undefined4 param_2);
undefined4 __cdecl FUN_004355ca(int param_1,uint param_2,int param_3);
void __cdecl FUN_0043586e(undefined4 param_1);
undefined4 __cdecl FUN_0043590c(uint **param_1,uint param_2,int param_3);
void __cdecl FUN_00435cfd(int param_1,uint param_2);
void __cdecl FUN_004363f8(int param_1,int param_2,int param_3);
void __cdecl FUN_0043667f(uint param_1,int param_2,int param_3,int param_4,uint param_5,int param_6,int param_7,int param_8);
uint __cdecl FUN_00436fe6(int param_1);
undefined4 __cdecl FUN_0043711a(uint **param_1,uint param_2,uint param_3);
int __cdecl FUN_004376d8(uint param_1);
int __cdecl FUN_00437726(int param_1,uint param_2);
int __cdecl FUN_004378b4(uint param_1);
int __cdecl FUN_00437902(int param_1,uint param_2);
int __cdecl FUN_00437b48(int param_1);
int __cdecl FUN_00437b96(int param_1,int param_2);
int FUN_00437e3e(void);
int __cdecl FUN_00437e91(int param_1);
void __cdecl FUN_00438010(int param_1,uint param_2);
int * __cdecl FUN_00438792(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,int param_9,int param_10,undefined4 param_11);
LPCSTR * __cdecl FUN_00438ad0(LPCSTR *param_1,byte param_2);
undefined4 __cdecl FUN_00438b9b(int param_1,undefined *param_2,uint param_3,int param_4,int param_5);
void __cdecl FUN_004390ae(int param_1,int param_2);
uint __cdecl FUN_0043915e(int param_1,uint param_2,int param_3);
undefined4 __cdecl FUN_0043920b(int param_1,int param_2,uint param_3);
undefined4 __cdecl FUN_00439289(int param_1,uint param_2);
undefined4 __cdecl FUN_0043936e(int param_1,uint *param_2);
undefined4 __cdecl FUN_00439407(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
undefined4 __cdecl FUN_00439559(int param_1,int param_2,int param_3,int param_4,int param_5);
undefined4 __cdecl FUN_00439c67(int param_1,int param_2,int param_3,int param_4);
void FUN_00439ce1(void);
void __cdecl FUN_00439d27(int param_1,int param_2);
void __cdecl FUN_00439f46(int param_1,int param_2);
void __cdecl FUN_0043a17b(int param_1,int param_2,uint param_3,uint param_4);
void __cdecl FUN_0043a213(int param_1,int param_2,int param_3,int param_4,uint param_5);
void __cdecl FUN_0043a2ac(int param_1,int param_2,int param_3);
void __cdecl FUN_0043a32d(int param_1,int param_2,uint param_3,int param_4,int param_5);
void __cdecl FUN_0043a3d6(int param_1,int param_2);
void __cdecl FUN_0043a541(undefined4 param_1,int param_2,int param_3,int param_4);
uint __cdecl FUN_0043a597(int param_1);
void __cdecl FUN_0043a681(int param_1,int param_2,int param_3);
void __cdecl FUN_0043a810(int param_1,int param_2);
int __cdecl FUN_0043a8a2(int param_1);
uint __cdecl FUN_0043a8d5(uint param_1,uint param_2);
undefined4 __cdecl FUN_0043a96a(int param_1,int param_2,int param_3,int param_4,int param_5);
undefined4 __cdecl FUN_0043ab53(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_0043ab95(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
int __cdecl FUN_0043ae39(undefined4 param_1);
uint FUN_0043ae7c(void);
undefined4 __cdecl FUN_0043b319(uint **param_1,uint param_2,uint param_3);
uint __cdecl FUN_0043b552(undefined4 param_1);
undefined4 __cdecl FUN_0043b651(uint **param_1,uint param_2,uint param_3);
uint __cdecl FUN_0043b7fd(int param_1,int param_2,int param_3);
uint __cdecl FUN_0043b921(int param_1,uint param_2);
undefined4 __cdecl FUN_0043b9ac(int param_1,int param_2,int param_3);
void __cdecl FUN_0043ba14(undefined4 *param_1,int *param_2,int param_3);
void __cdecl FUN_0043baeb(char *param_1,int param_2,int param_3,int param_4,int param_5);
undefined * FUN_0043bcea(void);
void __cdecl FUN_0043bd09(int param_1,int param_2);
void __cdecl FUN_0043be20(int param_1);
uint __cdecl FUN_0043bf7a(undefined4 param_1,int param_2,undefined4 param_3);
undefined4 FUN_0043c7a2(uint **param_1,uint param_2,uint param_3,int param_4);
uint __cdecl FUN_00443baa(int param_1);
void __cdecl FUN_00443c4d(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_00444385(uint param_1,uint param_2,int param_3,int param_4);
void __cdecl FUN_00444faf(int param_1);
void __cdecl FUN_00445021(int param_1,int param_2,int param_3);
void __cdecl FUN_0044587d(uint **param_1);
void __cdecl FUN_00445ba1(int param_1);
void __cdecl FUN_00445cc3(undefined *param_1);
void __cdecl FUN_00445d2f(int param_1);
void __cdecl FUN_00445e54(int param_1);
void FUN_00447024(void);
undefined4 __cdecl FUN_0044735b(undefined4 param_1);
void __cdecl FUN_0044738c(int param_1,int param_2,int param_3);
void __cdecl FUN_00447607(int param_1,undefined4 param_2,undefined4 param_3);
void __cdecl FUN_0044771e(int param_1,undefined4 *param_2,undefined4 param_3);
void __cdecl FUN_004477de(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0044783a(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00447997(int param_1);
void __cdecl FUN_00447b38(int param_1,int param_2,int param_3);
void __cdecl FUN_00447bc6(int param_1,int param_2,int param_3);
void __cdecl FUN_00447c1d(int param_1,int param_2,int param_3);
void __cdecl FUN_00448824(int param_1,int param_2,int param_3,undefined4 param_4,undefined4 param_5,uint param_6,undefined4 *param_7);
void __cdecl FUN_004493f9(undefined4 *param_1,char *param_2,uint param_3);
void __cdecl FUN_004494c2(undefined4 *param_1,char *param_2,uint param_3);
void __cdecl FUN_0044958b(undefined4 *param_1,char *param_2,uint param_3);
undefined4 __cdecl FUN_00449654(int param_1,uint param_2,uint param_3,uint param_4,uint param_5,uint param_6);
undefined4 * __cdecl FUN_004499d9(undefined4 *param_1,int *param_2,int param_3,int param_4);
void __cdecl FUN_00449bde(int param_1);
void __cdecl FUN_00449f24(int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5,undefined4 param_6,undefined4 param_7,ulonglong param_8);
void __cdecl FUN_0044a7dd(int param_1,undefined2 param_2,undefined2 param_3);
int __cdecl FUN_0044a87f(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_0044a8e3(int param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_0044aa04(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0044ab7b(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0044ac19(int param_1,uint *param_2,uint *param_3);
int __cdecl FUN_0044ace5(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0044add9(int param_1);
void __cdecl FUN_0044b0ba(int param_1);
void FUN_0044b1a8(void);
void __cdecl FUN_0044b3a4(uint param_1,int param_2);
undefined4 __cdecl FUN_0044b5e4(uint **param_1,uint param_2,uint param_3,int param_4);
void FUN_0044bf63(void);
void __cdecl FUN_0044c1ce(int param_1);
uint FUN_0044c451(void);
void __cdecl FUN_0044c578(uint **param_1,int param_2,int param_3,uint param_4);
void __cdecl FUN_0044c88d(int *param_1);
undefined4 __cdecl FUN_0044cd2e(uint **param_1,uint param_2,uint param_3,int *param_4);
uint __cdecl FUN_0044dd16(byte *param_1,int param_2);
int __cdecl FUN_0044df49(PCHAR param_1,int param_2);
undefined4 FUN_0044e131(void);
undefined4 __cdecl FUN_0044e201(int param_1,int param_2,int param_3,int param_4,int param_5);
int FUN_0044e442(void);
int FUN_0044e499(void);
void __cdecl FUN_0044e4f0(uint **param_1);
void FUN_0044e5a4(void);
void FUN_0044e6a8(void);
void __cdecl FUN_0044e9a9(uint **param_1,int param_2);
void __cdecl FUN_0044eaf1(int param_1);
undefined4 __cdecl FUN_0044ebe0(uint **param_1,uint param_2,uint param_3,undefined4 param_4);
void __cdecl FUN_0044f2e6(uint **param_1);
void FUN_0044f3a1(void);
void FUN_0044f4a5(void);
undefined4 __cdecl FUN_0044f83f(uint **param_1,uint param_2,uint param_3);
void FUN_0044fdf2(void);
undefined4 __cdecl FUN_0044fee1(uint **param_1,uint param_2,uint param_3);
uint FUN_004507f8(void);
undefined4 __cdecl FUN_004508ef(uint **param_1,uint param_2,uint param_3,int param_4);
uint FUN_00450b06(void);
undefined4 __cdecl FUN_00450bfd(uint **param_1,uint param_2,uint param_3);
int __cdecl FUN_00450d29(int param_1);
byte __cdecl FUN_00450dbf(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7,int param_8);
void __cdecl FUN_00451169(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_004512db(int param_1,int param_2);
int __cdecl FUN_0045144a(int param_1);
void __cdecl FUN_00451519(int param_1);
void __cdecl FUN_0045158f(int param_1);
undefined4 __cdecl FUN_004515ca(int param_1,int param_2);
int __cdecl FUN_00451658(int param_1);
undefined4 __cdecl FUN_0045172c(int param_1,int param_2);
undefined4 __cdecl FUN_004517dd(int param_1,int param_2);
bool __cdecl FUN_0045193a(int param_1);
bool __cdecl FUN_004519ed(int param_1);
bool __cdecl FUN_00451aa0(int param_1,int param_2);
undefined4 __cdecl FUN_00451b9e(int param_1);
int __cdecl FUN_00451c34(int param_1);
int __cdecl FUN_00451c91(int param_1);
void __cdecl FUN_00451d6d(int param_1,int param_2,int param_3);
void __cdecl FUN_0045209e(int param_1);
void __cdecl FUN_00452148(int param_1);
void __cdecl FUN_00452328(int param_1);
undefined4 __cdecl FUN_00452646(int param_1,int param_2,int param_3);
int __cdecl FUN_0045294f(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00452a43(int param_1);
void __cdecl FUN_00452ac3(int param_1);
int __cdecl FUN_00452b26(int param_1,int param_2);
undefined4 __cdecl FUN_00452bf8(int param_1,int param_2,int param_3);
undefined4 * __cdecl FUN_00452d01(int param_1);
undefined4 __cdecl FUN_00452e41(int param_1);
bool __cdecl FUN_00452ec6(int param_1);
void __cdecl FUN_00452f5b(int param_1);
void __cdecl FUN_00452fc9(int param_1);
void __cdecl FUN_00453057(int param_1,int param_2);
void __cdecl FUN_004530cd(int param_1,int param_2);
int __cdecl FUN_00453166(int param_1);
void __cdecl FUN_004531ac(int param_1);
int __cdecl FUN_00453247(int param_1,int *param_2);
void __cdecl FUN_00453383(int *param_1);
void __cdecl FUN_00453423(int *param_1,int param_2);
void __cdecl FUN_00453463(undefined4 param_1,undefined param_2);
void __cdecl FUN_0045349e(byte **param_1,int *param_2,int param_3);
void __cdecl FUN_0045383c(char **param_1,int *param_2);
undefined4 __cdecl FUN_00453a3c(byte *param_1,byte *param_2);
undefined4 __cdecl FUN_00453b2a(byte *param_1,byte *param_2);
void __cdecl FUN_00453c16(undefined4 param_1,undefined4 param_2);
undefined4 __cdecl FUN_004541bf(uint **param_1,uint param_2,int param_3);
int FUN_00454908(void);
void __cdecl FUN_00454cc4(char *param_1,int param_2);
int __cdecl FUN_00455070(int param_1,int param_2,int param_3);
undefined4 * FUN_00455143(void);
void __cdecl FUN_0045518a(uint param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,undefined4 *param_5,uint param_6,undefined2 param_7);
undefined4 FUN_0045532a(void);
int * __cdecl FUN_004553fd(int param_1,int param_2);
void FUN_00455648(void);
void FUN_0045571f(void);
uint __cdecl FUN_0045578c(undefined4 param_1,int param_2,uint param_3);
undefined4 __cdecl FUN_00455a3d(uint **param_1,uint param_2,uint param_3,int param_4);
uint __cdecl FUN_00455e22(char *param_1,int param_2,undefined4 param_3);
undefined4 __cdecl FUN_0045614c(uint **param_1,uint param_2,uint param_3,int param_4);
uint __cdecl FUN_004566ac(int *param_1);
uint __cdecl FUN_00456c19(int param_1,char *param_2,uint param_3);
undefined4 __cdecl FUN_00457028(uint **param_1,uint param_2,uint param_3,int param_4);
int __cdecl FUN_004574d6(char **param_1);
int __cdecl FUN_00457721(int *param_1);
undefined4 __cdecl FUN_004579ea(int param_1);
undefined4 __cdecl FUN_00457f10(int param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_00458e3a(int param_1,uint **param_2,int param_3);
undefined4 __cdecl FUN_004591f7(int param_1,uint **param_2);
void __cdecl FUN_00459247(undefined4 param_1,uint **param_2);
uint __cdecl FUN_00459479(int param_1,uint **param_2,int param_3,int param_4);
undefined4 __cdecl FUN_004598c9(int param_1,uint **param_2);
void __cdecl FUN_00459921(int param_1);
void __cdecl FUN_0045996c(int param_1);
int __cdecl FUN_004599b0(uint param_1,uint param_2,undefined4 param_3);
void __cdecl FUN_00459ca5(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00459d35(int param_1,int param_2,int param_3,int param_4,int param_5);
undefined4 __cdecl FUN_00459e8f(int param_1);
bool __cdecl FUN_0045a451(int param_1,uint param_2,uint param_3);
bool __cdecl FUN_0045a870(int param_1,uint param_2);
int __cdecl FUN_0045a902(undefined4 param_1,int param_2,uint param_3);
int __cdecl FUN_0045aa12(int param_1,int param_2,int param_3,int param_4);
undefined4 * __cdecl FUN_0045ad4e(int param_1,int param_2,int param_3);
undefined4 FUN_0045ae28(void);
uint __cdecl FUN_0045af67(int param_1,int param_2,int param_3,uint param_4,uint param_5,int param_6);
uint __cdecl FUN_0045b45b(int param_1,int param_2,int param_3,int param_4,uint *param_5,uint *param_6);
int __cdecl FUN_0045b75f(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0045b7ac(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0045bc82(int param_1);
void __cdecl FUN_0045bd09(int param_1);
uint __cdecl find_file_fn_0045bde0(byte *param_1,char *param_2,int param_3);
undefined4 __cdecl FUN_0045c087(uint **param_1,uint param_2,uint param_3,int *param_4);
uint ** __cdecl FUN_0045c3e4(uint **param_1,LPCSTR param_2,HANDLE param_3);
uint FUN_0045c549(void);
undefined4 __cdecl FUN_0045c663(uint **param_1,uint param_2,uint param_3,DWORD param_4);
undefined4 __cdecl FUN_0045d0c7(int param_1,int param_2,int param_3,int param_4,int *param_5);
void __cdecl FUN_0045d2af(int param_1,int param_2,int param_3);
void __cdecl FUN_0045d45a(int param_1);
void __cdecl FUN_0045d872(undefined4 *param_1,int param_2);
void __cdecl FUN_0045dc08(int *param_1,int param_2);
uint FUN_0045ddf0(void);
void FUN_0045e179(void);
undefined4 __cdecl FUN_0045e370(uint **param_1,uint param_2,uint param_3,int param_4);
void FUN_0045e87b(void);
void FUN_0045e963(void);
void FUN_0045e9de(void);
void FUN_0045ee03(void);
undefined4 __cdecl FUN_0045ef35(int param_1,uint param_2,uint param_3);
void FUN_0045f1a0(void);
undefined4 __cdecl FUN_0045f290(uint **param_1,uint param_2,int param_3);
void __cdecl FUN_0045f499(undefined4 param_1);
void FUN_0045f9d1(void);
void FUN_0045fa9c(void);
undefined4 __cdecl FUN_0045fbe0(uint **param_1,uint param_2,int param_3,undefined4 param_4);
char __cdecl FUN_00460210(int param_1,char **param_2);
char __cdecl FUN_00460296(int param_1,int *param_2);
int __cdecl FUN_0046031c(int param_1,int param_2);
void __cdecl FUN_004606b6(int param_1);
uint __cdecl FUN_00460b04(byte *param_1,int param_2,ushort *param_3);
uint FUN_00460b74(void);
uint __cdecl FUN_00461135(byte *param_1);
int __cdecl FUN_004611cf(int param_1,char **param_2,int param_3);
int __cdecl FUN_00461b77(int param_1,int *param_2);
undefined4 __cdecl FUN_004624f3(int param_1);
void __cdecl FUN_00462543(int param_1);
int __cdecl FUN_00462571(int param_1,int param_2,int param_3);
void __cdecl FUN_00462679(int param_1,int param_2);
void FUN_004626e0(void);
void __cdecl FUN_004626f4(int param_1,int param_2,int param_3,int param_4);
bool __cdecl FUN_0046292a(int param_1);
undefined4 __cdecl FUN_0046295d(int param_1,int param_2);
int __cdecl FUN_004629a4(int param_1);
void __cdecl FUN_004629cd(int param_1);
void __cdecl FUN_00462a28(int param_1,int param_2,int param_3,undefined param_4);
int __cdecl FUN_00462a5b(int param_1,int *param_2);
int __cdecl FUN_00462ae2(int param_1,char **param_2);
void __cdecl FUN_00462b69(undefined4 *param_1);
undefined4 __cdecl FUN_00462b8d(int *param_1,undefined4 param_2,undefined param_3);
int * __cdecl FUN_00462c31(int *param_1);
void FUN_00462d47(void);
void FUN_0046363e(void);
undefined4 __cdecl FUN_00463cd9(int param_1);
void FUN_00463f7b(void);
uint __cdecl FUN_00464119(undefined4 *param_1);
void FUN_00464c97(void);
void __cdecl FUN_00465456(undefined4 param_1);
undefined4 __cdecl FUN_004654df(int *param_1,int *param_2);
void __cdecl FUN_0046556d(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_004656d4(int param_1,int *param_2,int param_3);
undefined4 __cdecl FUN_00465996(int param_1,int *param_2,int param_3);
undefined4 __cdecl FUN_00465b5e(int param_1,int param_2,int param_3);
void __cdecl FUN_00465d24(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00465dae(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
undefined4 __cdecl FUN_00465f06(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7);
void __cdecl FUN_0046623d(int param_1,int param_2,int param_3);
void __cdecl FUN_00466271(undefined4 param_1,uint param_2,int param_3,int param_4,undefined4 param_5);
int * __cdecl FUN_004662c9(int param_1,int param_2,int param_3,uint param_4,int *param_5);
int __cdecl FUN_0046645d(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_004664c9(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
int FUN_004666a6(void);
undefined4 * FUN_0046672b(void);
void __cdecl FUN_00466d5a(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_00466de5(int param_1,int param_2,int param_3,uint param_4);
undefined4 FUN_004674aa(void);
char * __cdecl FUN_00467684(char *param_1);
bool __cdecl FUN_00467704(byte *param_1,int *param_2);
byte * __cdecl FUN_00467940(byte *param_1,int *param_2);
char * __cdecl FUN_004679f5(char *param_1,undefined4 *param_2);
void __cdecl FUN_00467aaf(int param_1,int param_2);
void __cdecl FUN_00467c94(int param_1,int param_2,int param_3,uint param_4);
bool __cdecl FUN_00468208(uint param_1,uint param_2,uint param_3);
void __cdecl FUN_00468353(int param_1,uint param_2);
void __cdecl FUN_00468b26(int param_1,int param_2);
bool __cdecl FUN_004690a0(uint param_1,uint param_2,int param_3);
void __cdecl FUN_004691f7(int param_1,uint param_2,uint param_3,int param_4,uint param_5);
void FUN_00469575(void);
int __cdecl FUN_0046a318(int param_1,int param_2);
void FUN_0046a4c3(void);
void __cdecl FUN_0046a6f9(int param_1);
void __cdecl FUN_0046a78d(int param_1);
void FUN_0046a831(void);
undefined4 __cdecl FUN_0046a873(int param_1,int param_2);
void __cdecl FUN_0046a941(int param_1);
void __cdecl FUN_0046aa15(int param_1,int param_2);
int __cdecl FUN_0046aa6f(int param_1,int param_2,int param_3,int param_4,int param_5);
int __cdecl FUN_0046abb5(int param_1);
void __cdecl FUN_0046ac65(int param_1);
void __cdecl FUN_0046ac91(int param_1);
void __cdecl FUN_0046acbd(int param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_0046ae9f(int param_1);
int __cdecl FUN_0046b2ed(uint param_1,int *param_2);
void __cdecl FUN_0046b5c5(int param_1,uint param_2);
undefined4 __cdecl FUN_0046b837(int param_1,uint param_2,uint *param_3,uint *param_4,int param_5);
undefined4 * __cdecl FUN_0046bdab(int param_1,int param_2,int param_3,int *param_4,undefined4 *param_5);
void FUN_0046be67(void);
undefined4 FUN_0046becf(void);
undefined4 __cdecl FUN_0046c0f5(int param_1,uint param_2);
void __cdecl FUN_0046c20d(uint param_1,int param_2);
undefined4 __cdecl FUN_0046c33d(int param_1,uint param_2,uint param_3,uint param_4,uint param_5);
int __cdecl FUN_0046c5b9(int param_1,int param_2);
void __cdecl FUN_0046c65e(int param_1,int *param_2,int param_3);
void __cdecl FUN_0046c8b5(int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,int param_5);
void FUN_0046cb74(void);
bool __cdecl FUN_0046d7b4(int param_1,int param_2,int param_3);
void __cdecl FUN_0046d88e(int param_1);
void __cdecl FUN_0046e218(undefined4 param_1,undefined4 param_2);
undefined4 __cdecl FUN_0046e49d(uint **param_1,uint param_2,int param_3);
uint __cdecl FUN_0046e816(int param_1,int param_2,int param_3);
int FUN_0046ef1d(void);
int __cdecl FUN_0046efbe(int param_1);
void FUN_0046f15b(void);
undefined4 __cdecl FUN_0046f354(int *param_1);
void __cdecl FUN_0046fa64(undefined4 param_1,undefined4 param_2,undefined4 param_3,char param_4,char param_5,int param_6,undefined param_7,undefined2 param_8,int param_9);
void __cdecl FUN_0046fba7(int param_1);
void FUN_0046fc49(void);
void FUN_0047051a(void);
undefined4 __cdecl FUN_00470ee2(int param_1,int param_2,undefined4 param_3,undefined4 param_4);
undefined4 FUN_00470ff6(void);
uint __cdecl FUN_004710a6(undefined4 param_1,int param_2,int param_3,int param_4,int param_5);
undefined __cdecl FUN_004713df(undefined4 param_1,int param_2,int param_3,int param_4,int *param_5,int *param_6);
void FUN_00471540(void);
int * __cdecl FUN_0047157e(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,undefined4 param_9);
LPCSTR * __cdecl FUN_004716c6(LPCSTR *param_1,byte param_2);
undefined4 __cdecl FUN_00471791(int param_1,undefined *param_2,uint param_3,int param_4,int param_5);
void FUN_00471b89(void);
void __cdecl FUN_00471bcd(int param_1);
void __cdecl FUN_00471e90(int param_1);
void FUN_00472186(void);
void FUN_00472275(void);
void __cdecl FUN_004722d3(int param_1);
int __cdecl FUN_00472441(undefined4 param_1,int param_2,int param_3);
void __cdecl FUN_004724b5(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0047254a(undefined4 param_1,undefined4 param_2);
undefined4 __cdecl FUN_00472580(undefined4 param_1,int param_2,int param_3);
undefined4 __cdecl FUN_004726ac(undefined4 param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_004729ba(int param_1,int *param_2,int *param_3,int param_4);
void __cdecl FUN_00472a32(int param_1);
int __cdecl FUN_00472a92(undefined4 param_1,int param_2,int param_3,int param_4);
uint __cdecl FUN_00472bb0(int param_1);
undefined4 __cdecl FUN_00473573(int param_1,uint param_2,int param_3);
void __cdecl FUN_00473663(int param_1,int param_2,int param_3);
void __cdecl FUN_00473798(undefined4 param_1,int param_2,int param_3,int param_4,int param_5);
undefined4 * __cdecl FUN_004738c9(int param_1,int param_2);
undefined * FUN_004739b3(void);
undefined4 * __cdecl FUN_004739d2(undefined4 *param_1);
LPCSTR * __cdecl FUN_004739f8(LPCSTR *param_1);
LPCSTR * __cdecl FUN_00473a39(LPCSTR *param_1,byte param_2);
undefined4 __cdecl FUN_00473a9c(undefined4 param_1);
uint FUN_00473acd(void);
void FUN_00476d51(void);
undefined4 __cdecl FUN_00476e7c(int param_1,uint param_2,int param_3,int param_4);
uint FUN_0047706a(void);
undefined4 __cdecl FUN_00477161(uint **param_1,uint param_2,int param_3);
undefined4 __cdecl FUN_0047722f(int param_1,int param_2);
void __cdecl FUN_00477c14(int param_1);
void __cdecl FUN_00477d4b(int param_1,int param_2,int param_3);
void __cdecl FUN_0047830a(uint **param_1);
undefined4 __cdecl FUN_004786a2(int param_1,int param_2,int param_3);
int FUN_004789fc(void);
void FUN_00478ab4(void);
undefined4 __cdecl FUN_00478d5b(undefined4 param_1);
undefined4 FUN_00479a92(void);
int __cdecl FUN_0047a593(undefined4 param_1);
int __cdecl FUN_0047bf4b(undefined4 param_1,undefined4 param_2,uint param_3);
void FUN_0047d42c(void);
DWORD __cdecl FUN_0047d5ab(int *param_1);
void __cdecl FUN_0047d608(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_0047d6a0(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0047d7d7(int param_1,int param_2,int param_3);
void __cdecl FUN_0047d83f(int param_1,undefined4 param_2,undefined4 param_3);
void __cdecl FUN_0047d8c7(int param_1,undefined4 param_2,undefined4 param_3);
void __cdecl FUN_0047dbde(int param_1);
void __cdecl FUN_0047dc83(int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5,uint param_6);
uint __cdecl FUN_0047de3c(uint *param_1,uint *param_2,uint *param_3,uint *param_4,uint param_5,uint param_6,uint param_7,uint param_8);
void __cdecl FUN_0047e204(int param_1,int param_2,int param_3);
void __cdecl FUN_0047e2a6(int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5,undefined4 param_6,undefined4 param_7,ulonglong param_8);
void __cdecl FUN_0047e980(int param_1,undefined4 param_2,undefined4 param_3,int param_4);
void __cdecl FUN_0047ec92(undefined4 param_1,int param_2,int param_3,int param_4,int param_5,uint param_6);
void FUN_0047ed66(void);
void __cdecl FUN_0047eda9(undefined4 param_1,uint param_2,int param_3,int param_4,int param_5,int param_6);
void __cdecl FUN_0047ef5e(int param_1,int param_2,int param_3);
void FUN_0047f0b3(void);
int FUN_0047f618(void);
void FUN_0047fb0a(void);
uint FUN_0047fc74(void);
undefined4 __cdecl FUN_0047fda8(uint **param_1,uint param_2,uint param_3,int param_4);
uint FUN_0048006a(void);
undefined4 __cdecl FUN_0048019e(uint **param_1,uint param_2,int param_3,int param_4);
uint FUN_00480382(void);
undefined4 __cdecl FUN_004804b3(int param_1,uint param_2,uint param_3,int param_4);
uint FUN_004806c5(void);
undefined4 __cdecl FUN_00480801(uint **param_1,uint param_2,uint param_3,int *param_4);
void FUN_00480f18(void);
void __cdecl FUN_00480faf(int param_1,uint param_2,int param_3,int param_4);
void FUN_004810d1(void);
void __cdecl FUN_00481152(int param_1);
int __cdecl FUN_004811e6(int *param_1);
void FUN_0048130e(void);
void __cdecl FUN_0048134d(undefined2 param_1,undefined2 param_2,undefined2 param_3);
void FUN_004813ca(void);
undefined4 * FUN_004815e3(void);
int __cdecl FUN_00481647(int param_1);
void __cdecl FUN_004816a2(int param_1,int param_2,int param_3);
undefined4 * __cdecl FUN_00481784(int param_1,int param_2,int param_3);
void __cdecl FUN_004817f9(int param_1,int param_2,short *param_3);
undefined4 __cdecl FUN_00481a44(int param_1);
int __cdecl FUN_00481a7f(int param_1,int param_2,int param_3);
int __cdecl FUN_00481b16(int param_1,int param_2,int param_3);
undefined4 __cdecl FUN_00481ba7(int param_1,int param_2,int *param_3,int *param_4);
int __cdecl FUN_00481c66(int param_1);
int __cdecl FUN_00481d4f(int param_1);
undefined4 __cdecl FUN_00481e14(int param_1,int param_2,int param_3);
void __cdecl FUN_00481fde(int param_1,int param_2,int param_3);
undefined4 * __cdecl FUN_004823c9(int param_1,int param_2,int param_3,int *param_4);
void FUN_00482491(void);
undefined4 __cdecl FUN_00482588(int param_1,int param_2);
int __cdecl FUN_004825ee(int param_1);
int __cdecl FUN_00482655(int param_1);
int __cdecl FUN_004826bc(int param_1);
void __cdecl FUN_0048273d(int param_1,undefined2 param_2);
undefined4 __cdecl FUN_00482796(int param_1);
void __cdecl FUN_00482843(undefined4 param_1,uint param_2,int param_3,int param_4,undefined *param_5);
void __cdecl FUN_00482919(int param_1,int param_2);
void __cdecl FUN_00482954(int param_1,int param_2);
void __cdecl FUN_00482992(int param_1,int param_2);
void FUN_004829cc(void);
void __cdecl FUN_00482b7e(int param_1,int param_2);
int __cdecl FUN_00482bd5(int param_1);
void __cdecl FUN_00482c8a(int param_1);
void __cdecl FUN_00482cea(int param_1,int param_2,int param_3);
void __cdecl FUN_00482d56(int param_1,int param_2);
void __cdecl FUN_00482e50(int param_1);
void __cdecl FUN_00483355(int param_1);
undefined4 __cdecl FUN_0048348e(uint **param_1,uint param_2,uint param_3);
void FUN_00483978(void);
void __cdecl FUN_00483b22(int param_1);
undefined4 FUN_00483da3(void);
undefined4 * __cdecl FUN_004840cd(undefined4 *param_1,undefined4 *param_2,int param_3);
undefined4 * __cdecl FUN_00484134(undefined4 *param_1,int param_2);
int __cdecl FUN_0048418d(int param_1);
void __cdecl FUN_004841ea(int **param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00484ae4(int param_1);
int * __cdecl FUN_00484b4e(int *param_1);
undefined4 * FUN_00485347(void);
void __cdecl FUN_004853b9(int param_1,int param_2,int param_3);
int ** __cdecl FUN_00485463(short param_1,undefined4 param_2);
int ** __cdecl FUN_00485a91(short param_1,undefined4 param_2);
undefined4 * FUN_00485cd6(void);
int __cdecl FUN_00485d4e(int param_1,int param_2,int param_3);
int __cdecl FUN_00485e3d(int param_1);
int __cdecl FUN_00485ea2(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_00485fe3(int param_1);
void __cdecl FUN_00486065(undefined2 *param_1);
void __cdecl FUN_0048616e(int param_1,int param_2,int param_3,int param_4,int param_5);
int __cdecl FUN_00486432(int param_1,int param_2,int param_3,int param_4);
void FUN_004864f7(void);
undefined4 * __cdecl FUN_004865ba(undefined4 *param_1);
int ** __cdecl FUN_004869cb(int param_1,int param_2);
undefined4 __cdecl FUN_00486a5a(int param_1,int param_2);
void __cdecl FUN_00486af8(int param_1,int param_2);
void __cdecl FUN_00486b30(int param_1,int param_2);
void __cdecl FUN_00486b6b(int param_1,int param_2);
undefined4 * __cdecl FUN_00486ba4(int param_1);
void FUN_00486d59(void);
int __cdecl FUN_00486e79(int param_1,int *param_2);
int __cdecl FUN_004874bc(int *param_1,int param_2);
int __cdecl FUN_004875c2(int *param_1);
int __cdecl FUN_00487685(char **param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_004881e3(int param_1,int param_2,int param_3);
void FUN_00488257(void);
void __cdecl FUN_004883c0(int param_1,int param_2,int param_3);
void __cdecl FUN_00488456(int param_1,int param_2);
int __cdecl FUN_004884c0(int param_1,int param_2);
undefined4 * __cdecl FUN_00488613(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_004886d4(int param_1,int param_2);
undefined4 __cdecl FUN_00488757(int param_1,int param_2);
void FUN_004887e9(void);
int __cdecl FUN_004888c0(int param_1,int param_2,int param_3,int param_4);
undefined4 __cdecl FUN_004889db(ushort *param_1,int param_2);
void __cdecl FUN_00488ec0(int param_1,int param_2);
bool __cdecl FUN_00488efd(int param_1,int param_2);
void __cdecl FUN_00488f5c(int *param_1);
undefined4 __cdecl FUN_00489176(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00489246(int param_1,int param_2);
undefined4 __cdecl FUN_004894bf(int param_1);
undefined4 __cdecl FUN_00489539(int param_1,int param_2,int param_3);
void __cdecl FUN_0048976a(int param_1,int param_2);
uint __cdecl FUN_00489d55(int param_1);
uint __cdecl FUN_0048c9aa(int param_1);
void __cdecl FUN_0048d183(int param_1,int param_2);
undefined4 __cdecl FUN_0048d209(uint **param_1,uint param_2,uint param_3);
undefined * __cdecl FUN_0048deed(uint **param_1);
void __cdecl FUN_0048e055(int param_1,int param_2);
void __cdecl FUN_0048e988(undefined *param_1);
void __cdecl FUN_0048e9e0(undefined *param_1);
int __cdecl FUN_0048eb27(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0048efa1(uint **param_1);
int __cdecl FUN_0048f0a0(int param_1);
void __cdecl FUN_0048f4cb(int param_1);
undefined * __cdecl FUN_0048f614(int param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_0048f678(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,uint param_7,int param_8,int param_9,int param_10);
void __cdecl FUN_0048f724(int param_1,int param_2);
void __cdecl FUN_0048f9f5(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_0048fe33(int param_1,int param_2);
void __cdecl FUN_0049009c(int param_1,int param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5,undefined4 param_6,undefined4 param_7,undefined4 param_8,ulonglong param_9);
void __cdecl FUN_004906c1(char *param_1,char *param_2,char param_3,int param_4,int param_5,int param_6);
void __cdecl FUN_00490a17(undefined4 param_1,undefined4 param_2,int param_3);
int __cdecl FUN_00490aa9(int param_1,int param_2,int param_3);
void FUN_00490b38(void);
void FUN_0049122c(void);
uint FUN_004915d2(void);
void FUN_00491a89(void);
void FUN_00491c54(void);
void FUN_00491e92(void);
void FUN_004920ad(void);
void FUN_00492287(void);
void FUN_004924c5(void);
void FUN_004926f4(void);
void __cdecl FUN_0049290c(byte *param_1);
void FUN_00492be3(void);
void FUN_00492fad(void);
void FUN_00493189(void);
void FUN_00493573(void);
void FUN_00493850(void);
void FUN_00493b56(void);
char * __cdecl FUN_00493d9c(char *param_1,int param_2);
char * __cdecl FUN_00493dfe(char *param_1);
void __cdecl get_priv_profile_ints_00493e33(int param_1,LPCSTR file_name);
void read_set_options_004940cb(void);
void FUN_00494639(void);
int __cdecl FUN_0049488e(byte *param_1);
int __cdecl FUN_004948ea(byte *param_1);
void FUN_00494949(void);
undefined4 __cdecl FUN_00494bbd(int param_1,int param_2,undefined4 param_3,undefined4 param_4);
void __cdecl FUN_00494c43(undefined4 *param_1,undefined4 *param_2);
void FUN_00494c6a(void);
void FUN_00494f54(void);
void __cdecl FUN_00495145(int param_1,int param_2);
void FUN_004951f3(void);
undefined4 __cdecl FUN_00495286(uint param_1);
void FUN_0049536f(void);
void FUN_004953d7(void);
void __cdecl FUN_00495438(undefined4 param_1,int param_2,int param_3);
void FUN_00495491(void);
void __cdecl FUN_004954f3(undefined4 param_1,undefined4 param_2);
void __cdecl FUN_00495520(undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,int param_5);
void __cdecl FUN_00495607(int param_1);
void __cdecl FUN_004958f8(undefined4 *param_1);
void __cdecl FUN_0049595c(int *param_1);
void __cdecl FUN_00495a10(undefined4 param_1,undefined4 param_2);
void FUN_00495a31(void);
DWORD __cdecl direct_draw_create_00495a72(HWND window_handle_1,int param_2,LPCSTR param_3,LPCSTR param_4);
void FUN_00495cd1(void);
undefined4 FUN_00495d65(int param_1);
DWORD __cdecl palette_fn_00495df2(DWORD param_1,DWORD param_2,DWORD param_3);
undefined4 FUN_0049607e(void);
void FUN_004960b0(void);
void FUN_004960df(void);
void __cdecl FUN_004960ff(int param_1);
undefined4 FUN_0049613c(void);
void FUN_0049621b(void);
void __cdecl FUN_00496263(int param_1,int param_2,int param_3);
bool __cdecl FUN_0049637b(char *param_1,undefined4 *param_2);
void FUN_004963db(void);
void __cdecl FUN_00496404(int param_1);
void __cdecl FUN_0049642b(int param_1,int param_2);
void __cdecl FUN_004965e4(int param_1,int param_2);
void __cdecl FUN_004967d7(undefined4 *param_1,int param_2,int param_3);
void FUN_00496863(void);
void __cdecl FUN_004968e7(int param_1,int param_2,int param_3,int param_4,undefined param_5);
void FUN_00496a10(void);
void __cdecl FUN_00496a3b(int param_1,int param_2,undefined param_3);
void __cdecl FUN_00496ac0(undefined4 *param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_00496c1f(int param_1,int param_2,undefined4 *param_3,int param_4,int param_5);
void __cdecl FUN_00496d7e(int param_1,int param_2,uint param_3,uint param_4,int param_5,uint param_6,uint param_7);
void __cdecl FUN_00496ee6(char *param_1,int param_2,int param_3,uint param_4,int param_5);
void __cdecl FUN_00497045(int param_1,int param_2,uint param_3,uint param_4,int param_5,int param_6,uint param_7);
LPCSTR __cdecl FUN_004971ad(LPCSTR param_1);
int __cdecl FUN_00497282(int param_1,ushort *param_2,int param_3);
void __cdecl FUN_00497330(int param_1,int param_2,int param_3,int param_4,undefined4 param_5,int param_6,undefined4 param_7);
void __cdecl FUN_00497567(int param_1,int param_2,int param_3,int param_4,undefined4 param_5,int param_6,undefined4 param_7,undefined4 *param_8,byte param_9);
char __cdecl FUN_00497765(uint param_1,int param_2,int param_3,undefined4 param_4,undefined4 *param_5);
void __cdecl FUN_00497896(int param_1,int param_2,int param_3,int param_4,undefined param_5);
int __cdecl FUN_00497c58(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7);
void __cdecl FUN_00498293(int *param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
void __cdecl FUN_0049848b(undefined *param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_004984ed(undefined *param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00498556(undefined *param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_004985c3(undefined *param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00498620(undefined *param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00498688(undefined *param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_004986f7(undefined *param_1,undefined *param_2,int param_3,int param_4);
void __cdecl FUN_0049875c(byte *param_1,int param_2,int param_3);
void __cdecl FUN_00498799(byte *param_1,undefined *param_2,int param_3,int param_4);
int * __cdecl FUN_004987d6(int *param_1,int param_2,int param_3,int param_4,int param_5);
int * __cdecl FUN_00498852(int *param_1,int *param_2);
void __cdecl FUN_004988ab(int *param_1);
int * __cdecl FUN_0049890a(int *param_1);
void __cdecl FUN_00498999(int param_1,int param_2,int param_3,int param_4);
void __cdecl FUN_00498a5b(int *param_1);
void FUN_00498ae4(void);
void __cdecl FUN_00498b2e(undefined4 *param_1);
undefined4 * __cdecl FUN_00498ba4(undefined4 *param_1,undefined4 param_2,int param_3,int param_4);
int ** __cdecl FUN_00498cf4(int **param_1);
bool FUN_00498d99(void);
int * __cdecl FUN_00498dce(int param_1,undefined4 param_2);
void __cdecl FUN_00498df5(LPCSTR param_1);
char ** __cdecl FUN_00498e10(char *param_1,byte *param_2);
int * __cdecl FUN_00498ee0(int *param_1,LPSTR param_2,LPSTR param_3);
char * __cdecl FUN_00499050(int *param_1,int param_2);
int * __cdecl FUN_004990e0(undefined4 *param_1,int param_2,char *param_3,byte *param_4);
undefined4 * __cdecl FUN_00499920(char **param_1);
void __cdecl FUN_00499b30(undefined4 param_1,LPCSTR param_2);
LPCSTR __cdecl FUN_00499c00(int param_1,byte *param_2,uint *param_3);
uint __cdecl FUN_00499f60(uint param_1);
int __cdecl FUN_00499fb0(int param_1);
int __cdecl FUN_00499ff0(int param_1);
undefined4 * __cdecl FUN_0049a030(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,undefined4 param_9,undefined4 param_10);
LPCSTR * __cdecl FUN_0049a1c0(LPCSTR *param_1,byte param_2);
void __cdecl FUN_0049a250(int param_1,uint param_2);
undefined4 __cdecl FUN_0049a270(int param_1,undefined *param_2,uint param_3,int param_4,int param_5);
void __cdecl FUN_0049a6b0(int param_1);
undefined * __cdecl FUN_0049a770(undefined *param_1,int param_2,undefined4 param_3,undefined4 param_4);
void __cdecl FUN_0049a800(undefined *param_1,int param_2);
void __cdecl FUN_0049a850(undefined *param_1,int param_2,undefined4 param_3,undefined4 param_4);
undefined * __cdecl FUN_0049a8a0(undefined *param_1);
undefined4 FUN_0049a9c0(void);
undefined4 __cdecl FUN_0049aaa0(int param_1,int param_2,int param_3,int param_4,undefined4 *param_5);
undefined4 FUN_0049ab40(void);
void __cdecl FUN_0049ab50(undefined4 param_1,byte param_2);
void __cdecl FUN_0049abd0(int *param_1,undefined *param_2,int param_3);
void __cdecl FUN_0049ad10(int *param_1);
undefined4 __cdecl FUN_0049ae00(int param_1,undefined4 param_2);
void __cdecl FUN_0049ae20(undefined *param_1);
void __cdecl FUN_0049ae40(undefined *param_1);
void __cdecl FUN_0049ae60(uint **param_1,int param_2,int param_3,int param_4);
int __cdecl FUN_0049aea0(uint **param_1,int param_2,int param_3);
void FUN_0049aee0(void);
void __cdecl FUN_0049af50(LPCSTR param_1);
uint __cdecl FUN_0049bb50(uint **param_1,int param_2);
void __cdecl FUN_0049bf40(int *param_1,undefined4 param_2);
undefined * __cdecl FUN_0049bf80(uint **param_1,int param_2,int param_3,int param_4,undefined4 param_5);
void __cdecl FUN_0049c140(int param_1,int param_2);
void __cdecl FUN_0049c160(uint **param_1,int param_2);
void __cdecl FUN_0049c1a0(uint **param_1,int param_2);
void __cdecl FUN_0049c2c9(uint param_1);
void __cdecl FUN_0049c2e0(PCHAR param_1,PCHAR param_2);
void FUN_0049c305(void);
void set_curr_dir_0049c320(void);
undefined1 * __cdecl FUN_0049c374(char *param_1);
int * __cdecl FUN_0049c4bd(char *str_param_1,byte *param_2);
int * __cdecl FUN_0049c52b(char *param_1,byte *param_2);
void set_curr_dir_0049c55d(void);
bool __cdecl FUN_0049c57b(char *param_1,undefined4 *param_2,uint param_3);
bool __cdecl FUN_0049c60b(char *param_1,undefined4 *param_2,uint param_3,int param_4);
bool __cdecl FUN_0049c67c(char *param_1,undefined4 *param_2,uint param_3);
bool __cdecl FUN_0049c728(char *param_1,undefined4 *param_2,uint param_3,int param_4);
char * __cdecl FUN_0049c7b5(char *param_1);
char * __cdecl FUN_0049c8a4(char *param_1);
int __cdecl FUN_0049c993(char *param_1,uint param_2);
undefined4 __cdecl FUN_0049ca0b(char *param_1,uint param_2);
uint __cdecl FUN_0049ca40(undefined4 *param_1);
uint __cdecl FUN_0049ca84(undefined4 *param_1,int param_2);
int __cdecl FUN_0049caac(int param_1);
void __cdecl FUN_0049cabc(char *param_1,uint param_2);
uint __cdecl FUN_0049cb70(undefined4 *param_1,int param_2);
int * __cdecl FUN_0049cc70(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,char *param_9);
void __cdecl FUN_0049cce0(int *param_1,char *param_2);
void __cdecl FUN_0049cda0(int param_1);
LPCSTR * __cdecl FUN_0049cef0(LPCSTR *param_1,byte param_2);
void __cdecl FUN_0049d2b0(undefined4 param_1,undefined4 param_2);
uint __cdecl FUN_0049d2e0(int param_1,uint param_2,char *param_3);
uint __cdecl FUN_0049dc40(undefined4 param_1,uint param_2,char *param_3,char *param_4);
void __cdecl FUN_0049e5a0(int param_1,int param_2,int param_3,int param_4,undefined param_5);
void __cdecl FUN_0049e640(int param_1,int param_2,int param_3,int param_4,undefined param_5,undefined param_6,undefined param_7,int param_8);
void __cdecl timer_func_0049e710(DWORD param_time_val_1);
undefined4 * __cdecl FUN_0049e750(undefined4 *param_1);
undefined4 __cdecl FUN_0049e7a0(int param_1,uint param_2,int param_3,int param_4);
undefined4 * FUN_0049ea90(void);
undefined4 __cdecl FUN_0049eae0(int param_1,undefined4 *param_2,uint param_3);
void __cdecl FUN_0049eb40(int param_1,undefined4 param_2);
undefined4 * __cdecl FUN_0049eb60(int param_1,undefined4 param_2,undefined2 param_3);
void __cdecl FUN_0049ebc0(uint **param_1,int param_2);
uint __cdecl FUN_0049ec50(uint *param_1);
uint __cdecl FUN_0049ec70(uint *param_1);
uint __cdecl FUN_0049ec90(uint *param_1);
uint __cdecl FUN_0049ecc0(uint *param_1);
undefined4 __cdecl FUN_0049ecf0(int param_1,int param_2);
undefined4 __cdecl FUN_0049ed30(LPCSTR *param_1);
uint __cdecl FUN_0049ed90(uint param_1);
void __cdecl FUN_0049edc0(undefined4 *param_1);
void __cdecl FUN_0049edd0(int *param_1);
undefined4 __cdecl FUN_0049ede0(int param_1);
undefined4 __cdecl FUN_0049edf0(int param_1,int param_2);
void __cdecl FUN_0049ee30(LPCSTR *param_1,LPCSTR param_2);
dword __cdecl create_window_fn_0049eee0(HINSTANCE hinstance_param_1,DWORD param_2,DWORD param_3,LPSTR param_4,LPCSTR str_class_name_5,LPSTR param_6);
int __cdecl FUN_0049f160(int param_1,int param_2,int param_3);
LRESULT win_func_0049_f380(HWND win_handle_1,uint msg_2,HWND wparam_3,LPARAM lparam_4);
void FUN_0049f610(void);
DWORD * __cdecl config_periodic_timer_0049f686(DWORD *param_1);
DWORD __cdecl config_periodic_timer_0049f6ee(DWORD param_1);
undefined4 __cdecl FUN_0049f713(int param_1);
undefined4 __cdecl FUN_0049f777(int param_1);
int __cdecl FUN_0049f7e6(int param_1);
void __cdecl FUN_0049f97a(int param_1);
void __cdecl FUN_0049f99b(int param_1,undefined4 param_2,undefined2 param_3,undefined4 param_4,undefined4 param_5);
DWORD __cdecl win_handle_func_0049fb83(int param_1);
DWORD __cdecl win_func_0049fe83(DWORD *param_1,DWORD *param_2,uint *param_3,int *param_client_x_coord_4,LONG *param_client_y_coord_5);
bool __cdecl FUN_004a02ae(undefined4 param_1);
void pop_err_msg_box_and_exit_004a02f5(LPSTR param_1);
void __cdecl check_win_handle_004a0396(LPSTR param_1);
undefined4 __cdecl FUN_004a0430(undefined4 param_1,undefined param_2,uint param_3);
char __cdecl FUN_004a0450(uint param_1,int param_2,int param_3,undefined4 param_4,undefined4 *param_5,uint param_6,int param_7,uint param_8,int param_9);
char __cdecl FUN_004a06b1(uint param_1,ushort *param_2);
undefined __cdecl FUN_004a06f6(int param_1);
int __cdecl FUN_004a070a(int param_1,int param_2,int param_3,uint *param_4,uint *param_5);
undefined4 __cdecl FUN_004a07e0(char *param_1,undefined4 *param_2);
undefined4 __cdecl FUN_004a08c5(char *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7,int param_8,int param_9);
undefined4 __cdecl FUN_004a0cd8(char *param_1,int param_2,int param_3);
int __cdecl FUN_004a11c0(int param_1);
void __cdecl FUN_004a11e2(void);
void __cdecl FUN_004a1243(void);
bool FUN_004a12c7(void);
void __cdecl FUN_004a15ad(void);
undefined4 __cdecl FUN_004a1651(void);
void FUN_004a1c7f(void);
void __cdecl FUN_004a1dc0(undefined4 *param_1,undefined4 *param_2,uint param_3);
int __cdecl FUN_004a1e20(byte *param_1,byte *param_2,int param_3);
int __cdecl FUN_004a1e60(byte *param_1);
void __cdecl FUN_004a27a0(int param_1);
LPCSTR __cdecl FUN_004a2831(uint param_1);
int ** __cdecl FUN_004a2874(int **param_1,int *param_2,int *param_3);
undefined4 * __cdecl FUN_004a28ef(undefined4 *param_1);
void __cdecl FUN_004a2965(int param_1);
undefined4 __cdecl FUN_004a2a3b(undefined4 *param_1,int *param_2,undefined4 *param_3);
ulonglong __fastcall FUN_004a2ae0(undefined4 param_1,uint param_2,uint *param_3,uint *param_4);
dword __cdecl FUN_004a2cab(LPSTR param_1,dword param_2,DWORD param_3,DWORD param_4);
undefined4 __cdecl FUN_004a2d6b(void);
int FUN_004a2ed0(void);
uint FUN_004a2edc(void);
void __cdecl FUN_004a2f00(undefined4 param_1);
int __cdecl FUN_004a2f10(byte *param_1,byte *param_2);
char * __cdecl FUN_004a2f60(char *param_1,int param_2,byte **param_3);
undefined4 __cdecl FUN_004a2ff0(undefined4 param_1,char *param_2,int param_3,int param_4);
void __cdecl FUN_004a36b0(int param_1);
void FUN_004a3800(void);
int __cdecl FUN_004a3840(char *param_1,char **param_2,int param_3,int param_4,int *param_5,ushort *param_6,int param_7);
DWORD __cdecl find_file_fn_004a3a00(LPCSTR file_name,HANDLE param_2,HANDLE *param_3);
DWORD __cdecl find_file_fn_004a3a94(HANDLE *find_file_handle);
void __cdecl create_proc_004a3b16(LPCSTR current_dir,LPCSTR app_name,LPSTR command_line);
int __cdecl FUN_004a3c10(int *param_1);
void __cdecl FUN_004a3c48(char *param_1,char *param_2,uint param_3);
LPCSTR * __cdecl mci_dev_fn_004a3db0(LPCSTR *param_1,char *str_param_2);
void __cdecl mci_dev_fn_004a4050(LPCSTR *param_1);
undefined2 * __cdecl FUN_004a4580(undefined4 param_1,byte *param_2);
void __cdecl FUN_004a4830(undefined4 param_1,LPCSTR *param_2);
uint __cdecl FUN_004a4c20(LPCSTR *param_1,int param_2);
MCIERROR __cdecl FUN_004a5130(LPCSTR *param_1,uint param_2);
int __cdecl FUN_004a5210(int param_1);
MCIERROR __cdecl FUN_004a52d0(int param_1);
int __cdecl FUN_004a5310(int param_1);
void __cdecl FUN_004a5420(LPCSTR param_1);
DWORD __cdecl move_file_004a5430(LPCSTR existing_file_name,LPCSTR new_file_name);
void __cdecl FUN_004a5dd0(undefined *param_1,int param_2,int param_3);
uint __cdecl FUN_004a5e40(int param_1);
void __cdecl FUN_004a5ec0(int param_1,int param_2);
void __cdecl FUN_004a5f20(int param_1);
int __cdecl FUN_004a5f80(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,undefined4 param_6,undefined4 param_7,undefined4 param_8,undefined4 param_9,undefined4 param_10,char *param_11,byte *param_12);
void __cdecl FUN_004a63d0(int param_1,int param_2);
void __cdecl FUN_004a6510(int param_1,int param_2,int param_3,int param_4,int param_5);
void __cdecl FUN_004a6720(int param_1,int param_2);
void FUN_004a6a00(void);
void __cdecl FUN_004a6bc0(int param_1,uint param_2,int param_3,int param_4);
undefined4 __cdecl FUN_004a6e10(int param_1,uint param_2,byte param_3,int param_4);
void entry(void);
int __cdecl FUN_004a70a2(int param_1,uint param_2,int param_3);
void __cdecl FUN_004a70ea(undefined4 *param_1);
int __cdecl FUN_004a70f8(int *param_1);
undefined4 __cdecl FUN_004a7132(undefined4 param_1,int param_2,undefined4 param_3);
uint __cdecl FUN_004a7160(undefined4 *param_1,uint param_2,int param_3,int *param_4);
void FUN_004a7399(void);
void FUN_004a73f7(void);
void FUN_004a756b(void);
void FUN_004a75a6(void);
void __cdecl FUN_004a75c2(int param_1,int param_2);
void __cdecl FUN_004a763f(undefined4 param_1,undefined4 param_2);
void __cdecl FUN_004a7670(undefined4 param_1);
void __cdecl FUN_004a7699(int param_1,int param_2);
void __cdecl FUN_004a76f9(int param_1);
void __cdecl FUN_004a778a(undefined4 *param_1);
void __cdecl FUN_004a7806(undefined4 *param_1);
void FUN_004a789b(void);
undefined4 __cdecl FUN_004a790e(undefined4 param_1);
undefined4 __cdecl FUN_004a7952(undefined4 param_1);
uint __cdecl FUN_004a7970(undefined4 *param_1,uint param_2,int param_3,char **param_4);
int FUN_004a7b70(void);
undefined4 * __cdecl FUN_004a7c00(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,undefined4 param_9,undefined4 param_10,undefined4 param_11,int param_12,undefined4 param_13);
void __cdecl FUN_004a8d00(int param_1);
void __cdecl FUN_004a8f10(int param_1,int param_2);
uint __cdecl FUN_004a9080(uint param_1,int *param_2);
DWORD __cdecl FUN_004a91d0(uint param_1);
uint __cdecl FUN_004a9250(byte **param_1);
uint __cdecl FUN_004a9320(byte **param_1);
LPVOID __cdecl FUN_004a9350(LPVOID *param_1);
uint __cdecl FUN_004a9420(byte *param_1,undefined4 *param_2);
int * __cdecl FUN_004a955c(byte *param_1,byte param_2,uint param_3,undefined4 param_4,uint param_5,int *param_6);
int * __cdecl FUN_004a9670(byte *param_1,byte *param_2,uint param_3);
void __cdecl FUN_004a96cc(byte *param_1,byte *param_2);
undefined4 * __cdecl FUN_004a96e4(undefined4 *param_1);
int * __cdecl FUN_004a9764(byte *param_1,byte *param_2,undefined4 *param_3);
int __cdecl FUN_004a9800(byte *param_1,undefined *param_2,int param_3);
byte * __cdecl FUN_004a9860(byte **param_1,byte *param_2,byte *param_3,int param_4);
void __cdecl FUN_004a98c0(byte *param_1,byte *param_2,byte **param_3,byte **param_4,byte **param_5,byte **param_6);
char * __cdecl FUN_004a9a00(char *param_1,char *param_2,int param_3);
char * __cdecl FUN_004a9a40(char *param_1);
void FUN_004a9a60(void);
void __cdecl FUN_004a9a64(UINT param_1);
void exit_func_004a9ab0(UINT exit_code);
double __cdecl FUN_004a9ae0(double param_1);
float10 FUN_004a9b12(void);
void __cdecl FUN_004a9b70(undefined4 *param_1,byte *param_2);
undefined4 __cdecl FUN_004aa04b(int *param_1,int param_2,int param_3);
undefined4 __cdecl FUN_004aa0ab(int *param_1,int param_2,int param_3,int param_4,int param_5);
undefined4 __cdecl FUN_004aa144(int *param_1,int *param_2);
uint __cdecl FUN_004aa20c(int *param_1,int *param_2,int *param_3);
int __cdecl FUN_004aa45a(int *param_1,int *param_2,int *param_3);
void __cdecl FUN_004aa59b(int *param_1,int *param_2,int *param_3);
DWORD __cdecl FUN_004aa690(undefined4 *param_1);
undefined4 __cdecl FUN_004aa700(int param_1,int *param_2);
void __cdecl FUN_004aa744(undefined4 *param_1);
undefined4 __cdecl FUN_004aa75c(int *param_1,int param_2,uint param_3);
void __cdecl FUN_004aa980(undefined4 param_1,byte *param_2,uint **param_3);
void __cdecl FUN_004aa9bc(undefined4 param_1,byte *param_2);
int __cdecl FUN_004aa9f0(int param_1);
void __cdecl FUN_004aaa04(undefined4 *param_1,int param_2,int param_3,int param_4,int param_5,int param_6);
void __cdecl FUN_004aaa6c(int param_1,int param_2,undefined4 *param_3,int param_4,int param_5,int param_6);
void __cdecl FUN_004aaae0(LPCSTR param_1);
void __cdecl FUN_004aaaf0(int *param_1);
LPCSTR __cdecl FUN_004aac00(uint param_1);
uint * __cdecl FUN_004aac10(uint param_1);
undefined4 __cdecl FUN_004aad20(undefined4 param_1);
void __cdecl FUN_004aad90(int param_1,int param_2,int param_3,int param_4,undefined param_5);
void __cdecl FUN_004aae04(int param_1,int param_2,undefined param_3);
void __cdecl FUN_004aae28(int param_1,int param_2,int param_3,int param_4,int param_5,uint param_6,uint param_7,int param_8,int param_9,uint param_10,uint param_11);
void __cdecl FUN_004aaf15(undefined *param_1,uint param_2,uint param_3,undefined *param_4,uint param_5,uint param_6);
void __cdecl FUN_004aaf98(char *param_1,int param_2,int param_3,uint param_4,int param_5,int param_6);
void __cdecl FUN_004ab060(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,uint param_7,int param_8,int param_9,uint param_10,uint param_11);
uint __cdecl read_file_func_004ab180(uint param_1,char *read_buffer,DWORD param_3);
DWORD __cdecl FUN_004ab390(uint param_1);
void __cdecl FUN_004ab3e4(int param_1,int param_2,int param_3,int param_4,undefined param_5);
int __cdecl FUN_004ab5b0(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7);
undefined4 * __cdecl FUN_004abdf0(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,undefined4 param_9,undefined4 param_10,int param_11);
DWORD __cdecl FUN_004abfc0(int param_1,char *param_2,uint param_3,uint param_4,byte *param_5);
void __cdecl FUN_004acd60(int param_1);
void __cdecl FUN_004acde0(int param_1);
void __cdecl FUN_004acfc0(int param_1);
undefined4 __cdecl FUN_004ad160(int param_1);
void __cdecl FUN_004ad210(int param_1);
uint * __cdecl FUN_004ad360(undefined4 *param_1,int param_2,undefined4 param_3,int param_4,int param_5,int param_6,int param_7,undefined4 param_8,undefined4 param_9,undefined4 param_10,uint param_11);
LPCSTR * __cdecl FUN_004ad4b0(LPCSTR *param_1,byte param_2);
uint __cdecl FUN_004ad550(uint **param_1,undefined *param_2,uint param_3,uint param_4,uint *param_5);
void __cdecl FUN_004ae0b0(int *param_1,int param_2);
void __cdecl FUN_004ae180(int *param_1,int param_2,int param_3);
void __cdecl FUN_004ae340(int param_1,int param_2,undefined4 param_3);
uint __cdecl FUN_004ae3d0(uint *param_1);
void __cdecl FUN_004ae406(undefined4 *param_1,int *param_2);
void __cdecl FUN_004ae4d3(char *param_1,int param_2);
bool __cdecl FUN_004ae7e3(undefined4 param_1);
uint __cdecl FUN_004ae82a(int param_1);
void FUN_004ae8a8(void);
void FUN_004ae8ec(void);
void __cdecl FUN_004ae978(LPSTR param_1,LPSTR param_2,uint **param_3);
char * __cdecl FUN_004ae9a0(char *param_1,char param_2);
void __cdecl FUN_004ae9c0(byte *param_1,uint param_2);
void __cdecl FUN_004ae9e4(byte *param_1,uint param_2,uint param_3);
uint __cdecl file_fn_004aea10(byte *file_name,uint param_2,uint param_3,uint **param_4);
undefined4 FUN_004aec6c(void);
void FUN_004aec74(void);
void __cdecl FUN_004aec78(int param_1);
undefined4 FUN_004aec88(int param_1);
void FUN_004aec98(void);
int * __cdecl FUN_004aec9c(int param_1,int *param_2,HMODULE module_handle);
void __cdecl get_mod_handle_fn_004aeee8(DWORD param_1,int *param_2);
undefined4 * FUN_004aefc0(void);
void __cdecl FUN_004af080(int param_1);
void FUN_004af0e0(void);
undefined1 * FUN_004af21c(void);
DWORD __cdecl FUN_004af2f0(undefined4 *param_1);
DWORD __cdecl set_file_pointer_004af420(uint param_1,int param_2,DWORD param_3);
DWORD __cdecl FUN_004af4b0(uint param_1);
undefined4 __cdecl FUN_004af560(undefined4 *param_1,char *param_2,int *param_3,int param_4,int param_5,undefined4 param_6);
void __cdecl FUN_004af7d0(int param_1);
undefined4 __cdecl FUN_004af840(int param_1);
int __cdecl FUN_004af950(int param_1,undefined *param_2,int param_3,int param_4);
void __cdecl FUN_004afbf0(int param_1);
void __cdecl FUN_004afc30(int param_1,int param_2);
void __cdecl FUN_004aff90(int param_1);
void __cdecl FUN_004b0070(int param_1,int param_2,uint param_3);
int __cdecl FUN_004b00f0(int param_1,int param_2,uint param_3);
void __cdecl FUN_004b03a0(int param_1,int param_2);
int ** __cdecl FUN_004b0530(char *param_1,int *param_2,int param_3,int *param_4);
int ** __cdecl FUN_004b0840(int **param_1);
undefined4 FUN_004b08b0(void);
DWORD __cdecl free_mem_004b08ec(LPVOID param_1);
void __cdecl FUN_004b094c(LPVOID param_1);
undefined4 __cdecl FUN_004b097e(int param_1,uint param_2,int param_3);
void __cdecl check_key_state_004b0e63(DWORD *param_1);
void FUN_004b0ef0(void);
void FUN_004b0ef8(void);
void __fastcall FUN_004b0f10(uint param_1,uint param_2);
void __fastcall FUN_004b0f47(uint param_1,undefined4 param_2);
void __cdecl FUN_004b0fb3(char *param_1,UINT param_2);
undefined4 FUN_004b0ff3(undefined4 param_1);
undefined4 FUN_004b1003(void);
undefined4 FUN_004b102f(int param_1);
int __cdecl FUN_004b1158(char *param_1,int param_2,char **param_3);
void __cdecl FUN_004b1278(uint param_1,int param_2);
undefined4 __cdecl FUN_004b1290(uint param_1);
void handle_err_fn_004b12fc(void);
void __cdecl file_time_fn_004b1310(FILETIME *file_time_to_convert,LPWORD fat_date,LPWORD fat_time);
void __cdecl FUN_004b1374(int param_1,undefined *param_2);
DWORD __cdecl find_file_fn_004b13bc(HANDLE find_file_handle,uint param_2,LPWIN32_FIND_DATAA find_file_data);
WORD __cdecl get_time_info_004b1400(uint *time_info);
int __cdecl FUN_004b1470(int *param_1);
DWORD __cdecl delete_file_fn_004b1630(LPCSTR file_name);
void FUN_004b1650(void);
void __cdecl FUN_004b1727(undefined4 *param_1);
void __cdecl FUN_004b1740(undefined4 param_1);
void FUN_004b175c(void);
void __cdecl FUN_004b1778(undefined4 param_1);
void __cdecl FUN_004b1790(undefined4 *param_1);
DWORD __cdecl write_file_004b1830(uint param_1,LPCVOID write_buf,DWORD num_bytes_to_write);
DWORD __cdecl read_file_004b1940(uint param_1,LPVOID read_buffer,DWORD num_bytes_to_read);
undefined4 __cdecl FUN_004b1a30(uint param_1);
void __cdecl FUN_004b1a88(int param_1,uint param_2);
void FUN_004b1ac0(void);
int __cdecl FUN_004b1acc(uint param_1);
uint FUN_004b1b20(void);
void __cdecl FUN_004b1b60(int param_1);
int __cdecl FUN_004b1ba0(byte *param_1,uint param_2);
char * __cdecl FUN_004b1be0(char *param_1,byte *param_2,int param_3);
int __cdecl FUN_004b1c10(byte *param_1,int param_2);
byte * __cdecl FUN_004b1c70(byte *param_1);
ushort __cdecl FUN_004b1ca0(byte *param_1);
void __cdecl FUN_004b1ce0(undefined **param_1);
void FUN_004b1cf0(void);
void __fastcall FUN_004b1d48(undefined4 param_1,byte param_2);
double __cdecl FUN_004b1dab(undefined4 param_1,undefined4 param_2);
float10 __cdecl FUN_004b1e09(byte *param_1,undefined4 *param_2);
void __cdecl FUN_004b1eb4(float10 *param_1,uint param_2);
void __cdecl FUN_004b1f0c(float10 *param_1,uint param_2);
void __cdecl FUN_004b1fa0(float10 *param_1,uint param_2);
uint __cdecl FUN_004b2008(uint *param_1,int *param_2,undefined4 *param_3);
void FUN_004b2317(void);
void FUN_004b232d(void);
void FUN_004b2344(void);
undefined4 * __cdecl FUN_004b24a7(undefined4 *param_1,undefined4 *param_2,undefined4 *param_3,int param_4,undefined4 *param_5);
void __cdecl FUN_004b26b8(int *param_1,undefined4 *param_2,int param_3,int param_4,undefined4 *param_5);
undefined4 __cdecl FUN_004b2a90(undefined4 *param_1,byte *param_2,uint **param_3);
DWORD __cdecl FUN_004b2b70(undefined4 *param_1);
DWORD __cdecl set_file_ptr_004b2b90(uint param_1);
void __cdecl FUN_004b2bf0(undefined **param_1);
void __cdecl FUN_004b2bfc(undefined4 param_1,int param_2);
int __cdecl FUN_004b2c10(undefined **param_1,byte *param_2,uint **param_3);
byte * __cdecl FUN_004b2edc(byte *param_1,int param_2);
int __cdecl FUN_004b2fd8(undefined **param_1);
int __cdecl FUN_004b3014(undefined **param_1,byte **param_2);
int __cdecl FUN_004b310c(undefined **param_1,int *param_2);
void __cdecl FUN_004b3270(int param_1,int *param_2,undefined4 param_3);
byte * __cdecl FUN_004b32dc(byte *param_1,int param_2);
int __cdecl FUN_004b3324(undefined **param_1,int *param_2,byte **param_3);
int __cdecl FUN_004b3438(undefined **param_1,uint **param_2);
int __cdecl FUN_004b373c(undefined **param_1,int **param_2,int param_3,int param_4);
int __cdecl FUN_004b3ba8(int param_1);
undefined4 __cdecl FUN_004b3bd8(undefined **param_1);
uint * FUN_004b3c10(void);
void FUN_004b3cc0(void);
int * __cdecl FUN_004b3df0(int *param_1);
uint * __cdecl alloc_mem_004b3e68(uint param_1);
void __cdecl FUN_004b3f18(uint param_1);
bool __cdecl FUN_004b3f2c(uint *param_1);
undefined4 FUN_004b3f80(void);
undefined4 FUN_004b3f90(void);
int __cdecl FUN_004b3fe0(int param_1);
void __cdecl set_std_handle_004b406c(HANDLE param_1,uint param_2);
void __cdecl FUN_004b4144(int param_1);
void get_std_handles_004b4170(void);
HANDLE create_event_fn_004b41d8(void);
void FUN_004b4244(void);
undefined4 __cdecl FUN_004b42c0(undefined4 param_1,byte *param_2,uint **param_3,undefined *param_4);
byte * __cdecl FUN_004b46e8(char *param_1,int **param_2,int param_3);
char * __cdecl FUN_004b484c(char *param_1,int param_2);
void __cdecl FUN_004b48bc(char *param_1,undefined4 param_2,int param_3);
int __cdecl FUN_004b48ec(ushort *param_1,undefined4 param_2,int param_3);
void __cdecl FUN_004b4968(uint param_1,char *param_2,int param_3);
void __cdecl FUN_004b49cc(char *param_1,uint param_2,int param_3);
void __cdecl FUN_004b4ad0(undefined4 param_1,undefined4 param_2,int param_3);
void __cdecl FUN_004b4b08(int param_1);
void __cdecl FUN_004b4b4c(ushort *param_1,undefined4 param_2,int param_3,undefined *param_4);
undefined8 __thiscall FUN_004b4bc4(void *this,ushort *param_1,uint **param_2,int param_3);
void __cdecl FUN_004b5228(byte *param_1);
void __cdecl FUN_004b5290(int param_1,undefined4 *param_2,undefined4 *param_3);
void __cdecl FUN_004b52d4(uint param_1,undefined4 *param_2);
DWORD __cdecl get_file_type_fn_004b5340(int param_1);
void FUN_004b53b0(void);
LPCRITICAL_SECTION init_crit_sec_004b53b4(void);
void FUN_004b5458(void);
void crit_sec_fn_004b5484(void);
void __cdecl FUN_004b54dc(int param_1);
void __cdecl enter_crit_sec_004b54f0(LPCRITICAL_SECTION *crit_sec_ptr);
void __cdecl leave_crit_sec_004b5554(LPCRITICAL_SECTION *param_1);
PDWORD tls_get_val_fn_004b568c(void);
int * __cdecl FUN_004b56cc(int *param_1);
bool alloc_tls_fn_004b570c(void);
int * __cdecl FUN_004b5768(int *param_1);
void __cdecl check_set_tls_val_004b57c4(int param_1);
BOOL tls_fn_004b5818(void);
void FUN_004b5824(void);
void FUN_004b5844(void);
LPCSTR __cdecl get_mod_file_name_fn_004b59f0(HMODULE module_handle,LPWSTR file_name,DWORD size);
undefined4 * __cdecl FUN_004b5aa0(LPSTR param_1);
undefined4 * __cdecl FUN_004b5af0(LPWSTR param_1);
void __cdecl query_virt_mem_004b5b30(int *param_1,int *param_2);
bool call_get_active_window_004b5ba0(void);
void __cdecl FUN_004b5bd8(char *param_1,char *param_2,uint param_3);
DWORD write_err_to_file_004b5c34(uint **param_1);
DWORD __cdecl err_handling_fn_004b5e68(PEXCEPTION_RECORD param_1,DWORD param_2,PCONTEXT param_3);
void __cdecl set_unhandled_except_filter_004b6038(undefined4 param_1);
void FUN_004b6084(void);
int __cdecl FUN_004b616c(int param_1);
undefined4 __cdecl FUN_004b61d0(LPCSTR param_1,byte param_2);
int FUN_004b6200(void);
byte * __cdecl FUN_004b6220(byte *param_1);
undefined4 * __cdecl get_file_path_fn_004b6280(undefined4 *path_name_buf,byte *file_name,uint buffer_len);
undefined4 * __cdecl FUN_004b6340(undefined4 *param_1,uint param_2);
DWORD __cdecl flush_file_buffers_fn_004b63f0(uint param_1);
undefined4 __cdecl FUN_004b6450(int param_1,int param_2,int *param_3);
void write_to_file_004b6760(char *param_1,UINT param_2);
void __cdecl FUN_004b67a0(char *param_1,UINT param_2);
undefined4 __cdecl FUN_004b67d0(undefined4 param_1,undefined2 param_2);
undefined4 __cdecl FUN_004b6800(uint param_1);
int __cdecl FUN_004b6838(int param_1,int param_2);
undefined4 __cdecl FUN_004b6938(int param_1,int param_2,int param_3);
uint __cdecl FUN_004b6994(int *param_1);
undefined4 __cdecl FUN_004b6c68(int *param_1,int *param_2);
undefined4 * __cdecl FUN_004b6ca0(int param_1,uint param_2,int param_3,undefined4 *param_4);
void __cdecl FUN_004b6de8(uint *param_1,undefined4 *param_2);
void time_zone_fn_004b6f68(void);
void __cdecl FUN_004b6f88(byte *param_1,int *param_2);
undefined4 * __cdecl FUN_004b6fb8(undefined4 *param_1,undefined4 *param_2,int *param_3);
char * __cdecl FUN_004b70d4(byte *param_1,int *param_2);
void __cdecl FUN_004b71dc(undefined4 *param_1);
undefined * FUN_004b7330(void);
void FUN_004b7340(void);
uint __cdecl read_console_input_004b7350(HANDLE param_1);
uint console_fn_004b7468(void);
undefined4 __cdecl write_to_console_004b7500(undefined4 param_1);
undefined4 __cdecl FUN_004b7570(byte *param_1);
undefined4 __cdecl FUN_004b75b0(byte *param_1);
char * __cdecl FUN_004b75e0(char *param_1,char *param_2,uint param_3);
double __cdecl FUN_004b7670(undefined4 param_1,undefined4 param_2,double *param_3);
void __cdecl FUN_004b7690(int param_1);
byte * __cdecl FUN_004b76d3(byte *param_1,int param_2,byte param_3);
void __cdecl FUN_004b7727(int param_1,byte *param_2);
undefined8 __cdecl FUN_004b781c(byte *param_1,double **param_2,undefined4 *param_3);
void FUN_004b79e0(void);
undefined4 __cdecl FUN_004b79f0(byte *param_1,undefined4 *param_2,byte **param_3);
double __cdecl FUN_004b7bb2(byte *param_1,byte **param_2);
undefined4 __cdecl FUN_004b7c98(int *param_1);
char * __cdecl FUN_004b7ce6(char *param_1,int param_2);
void __cdecl FUN_004b7d24(undefined4 param_1,undefined4 param_2,int param_3,undefined4 *param_4,undefined4 *param_5);
void __cdecl FUN_004b7d60(undefined4 param_1,undefined4 param_2,int param_3,int *param_4,undefined4 *param_5);
uint __cdecl conv_pchar_pwchar_004b7de0(WCHAR *param_1,byte *multi_byte_str,uint param_3);
uint __fastcall FUN_004b7e8c(undefined4 param_1,uint param_2);
ulonglong __fastcall FUN_004b7ed8(int param_1,int param_2);
void __cdecl FUN_004b7f20(uint *param_1,uint param_2);
uint * __cdecl FUN_004b7f38(uint *param_1,uint param_2);
DWORD __cdecl conv_pwchar_to_pchar_004b7fd0(LPSTR param_1,LPWSTR param_2);
char * __cdecl FUN_004b8010(uint param_1,char *param_2,uint param_3);
char * __cdecl FUN_004b8068(uint param_1,char *param_2,uint param_3);
char * __cdecl FUN_004b80a0(longlong *param_1,char *param_2,int param_3);
char * __cdecl FUN_004b81b0(uint param_1,char *param_2,uint param_3);
char * __cdecl FUN_004b8204(uint param_1,char *param_2,uint param_3);
void __cdecl FUN_004b8240(int param_1,int param_2);
LPVOID get_tls_val_004b8270(void);
undefined4 * FUN_004b82a8(void);
undefined4 __cdecl FUN_004b8380(undefined4 param_1,int param_2);
void __cdecl FUN_004b83ec(int param_1);
void FUN_004b8440(void);
void FUN_004b8464(void);
void __cdecl FUN_004b84a0(int *param_1);
void thunk_FUN_004b8464(void);
void __cdecl FUN_004b84e0(uint param_1);
LPCSTR __cdecl FUN_004b8630(short *param_1);
undefined4 * __cdecl FUN_004b8660(undefined4 *param_1,undefined4 *param_2);
undefined4 __cdecl FUN_004b8690(short *param_1);
void open_files_004b86bc(void);
undefined4 FUN_004b8730(void);
undefined4 FUN_004b873c(void);
void FUN_004b8750(void);
undefined4 __cdecl FUN_004b8760(int param_1,undefined4 param_2);
undefined4 __cdecl FUN_004b87b4(int param_1);
undefined4 __cdecl FUN_004b87d8(int param_1);
undefined4 HandlerRoutine_004b8820(int param_1);
undefined4 FUN_004b8880(void);
char set_console_ctrl_handler_004b88b8(void);
bool set_console_ctrl_handler_004b88e4(void);
undefined4 __cdecl FUN_004b8928(undefined4 param_1);
undefined4 __cdecl FUN_004b8968(int param_1,int param_2);
undefined4 __cdecl FUN_004b8a28(int param_1);
int __cdecl FUN_004b8b30(byte *param_1,byte *param_2,int param_3);
void __cdecl FUN_004b8bc0(short *param_1,int param_2);
void __cdecl FUN_004b8c40(char *param_1,int param_2);
void __cdecl FUN_004b8cd0(short *param_1,int param_2);
void __cdecl FUN_004b8d80(short *param_1,int param_2);
void __cdecl FUN_004b8ea0(undefined4 *param_1);
void FUN_004b8fe0(void);
void __cdecl FUN_004b9280(byte *param_1,int param_2);
undefined4 __cdecl FUN_004b95c0(undefined4 *param_1);
DWORD __cdecl FUN_004b9600(undefined4 *param_1,ushort *param_2,uint param_3);
undefined4 __cdecl FUN_004b96d0(UINT param_1);
void __fastcall FUN_004b9838(undefined4 param_1,undefined4 *param_2);
void __fastcall FUN_004b989a(undefined4 param_1,uint param_2);
int FUN_004b98e0(void);
undefined4 * __cdecl FUN_004b98ec(undefined4 param_1,undefined4 param_2,int param_3,undefined4 *param_4,undefined4 *param_5,int param_6,undefined4 *param_7);
int __cdecl FUN_004b9970(int param_1);
undefined4 __cdecl FUN_004b9998(short param_1,int *param_2,uint param_3,uint *param_4);
int * __cdecl FUN_004b9b54(int *param_1,uint param_2);
undefined8 __fastcall FUN_004b9bf3(uint param_1,uint param_2);
undefined4 FUN_004b9c90(void);
void FUN_004b9cb8(void);
void __cdecl FUN_004b9cc0(undefined4 param_1);
void FUN_004b9d20(void);
uint __cdecl FUN_004b9f00(uint param_1,uint param_2);
void __cdecl FUN_004b9f50(byte *param_1,byte *param_2);
void __cdecl set_handle_fn_004b9fd0(HANDLE *handles);
HANDLE __cdecl create_thread_fn_004ba070(DWORD param_1,int param_2,DWORD param_3);
void exit_from_thread_004ba168(void);
undefined * __cdecl FUN_004ba190(byte *param_1,byte **param_2,int param_3,int param_4);
void __cdecl FUN_004ba31c(byte *param_1,byte **param_2,int param_3);
int __cdecl FUN_004ba338(byte param_1);
int __cdecl FUN_004ba3a0(undefined4 *param_1);
undefined4 __cdecl FUN_004ba56c(byte *param_1);
int __cdecl FUN_004ba710(byte *param_1,int param_2);
void __cdecl FUN_004ba820(byte *param_1,byte *param_2);
byte * __cdecl FUN_004ba860(byte *param_1);
int __cdecl FUN_004ba8d0(byte *param_1,byte *param_2);
byte * __cdecl FUN_004ba930(byte *param_1,int param_2);
int __cdecl FUN_004ba9a0(byte *param_1);
uint __cdecl FUN_004ba9d0(WCHAR *param_1,byte *param_2,int param_3);
undefined4 __cdecl FUN_004bac04(ushort *param_1);
int __cdecl FUN_004bada4(ushort *param_1,int param_2);
void __cdecl FUN_004baec0(uint param_1);
void __cdecl FUN_004baf40(undefined4 param_1,char *param_2);
short * __cdecl FUN_004baf60(short *param_1,short param_2);
undefined4 * __cdecl FUN_004baf90(ushort *param_1,undefined4 *param_2);
void FUN_004bb0a0(void);
int __cdecl FUN_004bb110(int param_1);
uint __cdecl FUN_004bb130(uint param_1);
byte __cdecl FUN_004bb150(uint param_1);
undefined4 __cdecl FUN_004bb1a0(uint param_1);
uint __cdecl FUN_004bb1f0(uint param_1);
HRESULT DirectDrawCreate(GUID *lp_guid,LPDIRECTDRAW *lp_lp_dd,IUnknown *p_unk_outer);

