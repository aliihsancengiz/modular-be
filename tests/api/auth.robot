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
${USERNAME}		admin
${PASSWORD}		admin

${SNMP_USERS_API_PATH}	/api2/system/snmp/users



*** Test Cases ***

Login As Admin
	${auth}=	Create Dictionary	username=${USERNAME}	password=${PASSWORD}
	${headers}=	Create Dictionary	Content-Type=application/json
	${r}=	POST On Session	API	/api/authentication	json=${auth}	headers=${headers}	expected_status=200

Try Login With Invalid Credentials
	${auth}=	Create Dictionary	username=${USERNAME}	password=WrongPassword
	${headers}=	Create Dictionary	Content-Type=application/json
	${r}=	POST On Session	API	/api/authentication	json=${auth}	headers=${headers}	expected_status=401

