*** Settings ***
Library		Collections
Library		RequestsLibrary
Library		OperatingSystem
Library		Process
Library		DateTime
Library		JSONLibrary

Suite Setup	Create Session	API	${URL}	disable_warnings=true	verify=False
Suite Teardown	Delete All Sessions

*** Variables ***
${DUT}			127.0.0.1
${PORT}			8080
${URL}			http://${DUT}:${PORT}
${ADMIN_USERNAME}		admin
${ADMIN_PASSWORD}		admin

${SNMP_USERS_API_PATH}	/api2/system/snmp/users



*** Test Cases ***

Get Auth Token For Admin
	${auth}=	Create Dictionary	username=${ADMIN_USERNAME}	password=${ADMIN_PASSWORD}
	${headers}=	Create Dictionary	Content-Type=application/json
	${r}=	POST On Session	API	/api/authentication	json=${auth}	headers=${headers}	expected_status=200
	${Token}=	Evaluate	json.loads('''${r.content}''')	json
	Set Global Variable	${Token}

Check Only Admin Exists
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}
	${r}=	GET On Session	API	/api/user	headers=${Headers}	expected_status=200
	${Users}=	Evaluate	json.loads('''${r.content}''')	json
	${length}=	Get Length	${Users}
	Should Be Equal As Numbers	${length}	1
	Should Be Equal As Strings	${Users}[0][username]	admin
	Should Be Equal As Strings	${Users}[0][email]	admin@admin.com
	Should Be Equal As Strings	${Users}[0][role]	ADMIN

Create User
	${Param}=	Create Dictionary	username=user1	password=user1	email=user@user.com	role=USER
	${Param}=	Convert JSON To String	${Param}
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}	Content-Type=application/json
	${r}=	POST On Session	API	/api/user	headers=${Headers}	data=${Param}	expected_status=200
	${Users}=	Evaluate	json.loads('''${r.content}''')	json

Check User
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}
	${r}=	GET On Session	API	/api/user	headers=${Headers}	expected_status=200
	${Users}=	Evaluate	json.loads('''${r.content}''')	json
	${length}=	Get Length	${Users}
	Should Be Equal As Numbers	${length}	2
	Should Be Equal As Strings	${Users}[1][username]	user1
	Should Be Equal As Strings	${Users}[1][email]	user@user.com
	Should Be Equal As Strings	${Users}[1][role]	USER

Delete User
	${Param}=	Create Dictionary	username=user1	email=""	password=""	role=""
	${Param}=	Convert JSON To String	${Param}
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}	Content-Type=application/json
	${r}=	DELETE On Session	API	/api/user	headers=${Headers}	data=${Param}	expected_status=200

Create Viewer
	${Param}=	Create Dictionary	username=viewer1	password=viewer1	email=viewer@viewer.com	role=VIEWER
	${Param}=	Convert JSON To String	${Param}
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}	Content-Type=application/json
	${r}=	POST On Session	API	/api/user	headers=${Headers}	data=${Param}	expected_status=200
	${Users}=	Evaluate	json.loads('''${r.content}''')	json

Check Viewer
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}
	${r}=	GET On Session	API	/api/user	headers=${Headers}	expected_status=200
	${Users}=	Evaluate	json.loads('''${r.content}''')	json
	${length}=	Get Length	${Users}
	Should Be Equal As Numbers	${length}	2
	Should Be Equal As Strings	${Users}[1][username]	viewer1
	Should Be Equal As Strings	${Users}[1][email]	viewer@viewer.com
	Should Be Equal As Strings	${Users}[1][role]	VIEWER

Delete Viewer
	${Param}=	Create Dictionary	username=viewer1	email=""	password=""	role=""
	${Param}=	Convert JSON To String	${Param}
	${Headers}=	Create Dictionary	Authorization=Bearer ${Token["token"]}	Content-Type=application/json
	${r}=	DELETE On Session	API	/api/user	headers=${Headers}	data=${Param}	expected_status=200

