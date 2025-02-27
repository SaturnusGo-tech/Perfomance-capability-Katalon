import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import groovy.json.JsonSlurper as JsonSlurper

//GlobalVariable.memberId[0] = '3f971545-ad3f-4f74-b838-d909e752120b'
// Updated member buy BULK request
WebUI.comment('TEST CASE: Member BULK update')
WebUI.comment("0 ID IS : " + GlobalVariable.memberId[0] + ' AND ' + GlobalVariable.memberType[0])

GlobalVariable.firstName = GlobalVariable.firstName + 'BulkUPD'
WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Customer/Members/MemberUpdateBulk', [
	('id1') : GlobalVariable.memberId[0],
	('name1') : GlobalVariable.firstName,
	('id2') : GlobalVariable.memberId[1],
	('name2') : GlobalVariable.firstName,
	('memberType') : GlobalVariable.memberType[0],
	]))


// Check updated member
//responseGet = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Customer/Members/MemberGetId', [('id') : GlobalVariable.memberId[0]]))
//WS.verifyElementPropertyValue(responseGet, 'fullName', 'Qwe BulkUpd')

