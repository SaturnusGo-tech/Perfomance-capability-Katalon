import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import groovy.json.JsonSlurper
import internal.GlobalVariable
@Grab('com.xlson.groovycsv:groovycsv:1.3')
import static com.xlson.groovycsv.CsvParser.parseCsv



WebUI.comment("TEST CASE : Create new organization and set outer id")
//add extracting the outerId value from the csv at

GlobalVariable.firstName = 'agencyAuto'
GlobalVariable.lastName = 'agencyAuto'
GlobalVariable.contactName = 'agencyAuto'
def responseMemberCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Customer/Members/MemberCreate', [
	('memberType') : GlobalVariable.memberType[0],
	('name') : GlobalVariable.firstName,
	('firstName') : GlobalVariable.firstName,
	('lastName') : GlobalVariable.lastName,
	('fullName') : GlobalVariable.contactName
	]))
def memberJson = new JsonSlurper().parseText(responseMemberCreate.getResponseBodyContent())
GlobalVariable.memberId = memberJson.id


'UPDATE ORGANIZATION ADDRESSES WITH DEFAULT BILLING AND SHIPPING ADDRESSES'
addressTypes = ['Billing','Shipping']
List <Object>addressesList = []
for (int b; b < addressTypes.size(); b++) {
	addressDataTemplate = new File('TestDataFiles/addressDataTemplate.json').text
	
	addressDataTemplateDefault = new JsonSlurper().parseText(addressDataTemplate)
	addressDataTemplateDefault.isDefault = 'true'
	addressDataTemplateDefault.addressType = addressTypes.get(b)
	
	
	addressDataDefault = new groovy.json.JsonBuilder(addressDataTemplateDefault)
	addressesList.add(addressDataDefault)
	addressesString = addressesList.toString()
	String regex = "\\[|\\]"
	addresses = addressesString.replaceAll(regex, "");
	System.out.println(addresses);
	
	organizationUpdate =  WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Customer/Members/MemberUpdate', [
		('id') : GlobalVariable.memberId,
		('memberType') : GlobalVariable.memberType[0],
		('name') : GlobalVariable.firstName,
		('addresses')  : addresses
		]))
	 }
	 
	 
WebUI.comment("TEST CASE : Create contact in the organization")
def responseMemberCreateInOrg = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Customer/Members/MemberCreateInOrg', [
	('memberType') : GlobalVariable.memberType[1],
	('orgId') : GlobalVariable.memberId
	]))
def memberJson2 = new JsonSlurper().parseText(responseMemberCreateInOrg.getResponseBodyContent())
GlobalVariable.contactId = memberJson2.id


WebUI.comment('TEST CASE : Check Contact in Org')
responseContactsGet = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Customer/Members/MemberGetId', [
	('id') : GlobalVariable.contactId
	]))
WS.verifyElementPropertyValue(responseContactsGet, 'organizations[0]', GlobalVariable.memberId)


WebUI.comment("TEST CASE : Create user in account-contact")
Random rnd = new Random()
WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/UserCreate', [
	('email') : rnd.nextInt(100)+'@email.com',
	('userName') : GlobalVariable.agencyUserName,
	('contactId') : GlobalVariable.contactId,
	('storeId') : 'opus'
	]))


'SET USER PASSWORD FOR THE CREATED VENDOR'
passwordReset = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/UserPassword/UserPasswordReset',[
	('newPassword') : GlobalVariable.katalonVendorPassword,
	('userName') : GlobalVariable.agencyUserName
	]))
WS.verifyElementPropertyValue(passwordReset, 'succeeded', true)


'GET OUTER ID FROM THE IMPORTED CSV FILE'
file = new File('TestFiles/agenciesImport.csv')
fileText = file.getText()
fileParsed = parseCsv(fileText, separator: ',', readFirstLine: false)
fileContent = fileParsed[0]
outerId = fileContent[0] 


'GET THE CREATED ORGANIZATION'
organizationGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Customer/Members/MemberGetId',[
	('id') : GlobalVariable.memberId
	]))


'GET THE RESPONSE BODY AND UPDATE OUTER ID VALUE'
organizationBody = organizationGet.getResponseBodyContent()
organizationBodyParsed = new JsonSlurper().parseText(organizationBody)
organizationBodyParsed.outerId = outerId
organizationBodyJson = new groovy.json.JsonBuilder(organizationBodyParsed).toString()


'SEND REQUEST TO UPDATE THE ORGANIZATION WITH THE OUTER ID'
RequestObject organizationUpdateObject = findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Customer/Members/MemberUpdate')
organizationUpdateObject.setBodyContent(new HttpTextBodyContent(organizationBodyJson))
oranizationUpdate = WS.sendRequestAndVerify(organizationUpdateObject)

