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
import groovy.json.JsonSlurper as JsonSlurper



WebUI.comment('TEST CASE: add apiKey to the created user')
//This test case is required to separate the initial admin 
//from the generic platform user apiKey processing flow
//so the initial admin and its apiKey, that validates most requests 
//of the coverage, are not affected


'GENERATE APIKEY VALUE'
GlobalVariable.userApiKey = UUID.randomUUID()


'ADD API KEY TO THE CREATED USER'
WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/ApiKeySet', [
	('userName') : GlobalVariable.userName,
	('api_key') : GlobalVariable.userApiKey,
	('userId') : GlobalVariable.userId,
	('apiKeyStatus') : GlobalVariable.apiKeyStatus
	]))

responseApiKey = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/ApiKeyGet', [
	('userId') : GlobalVariable.userId
	]))
WS.verifyElementPropertyValue(responseApiKey, '[0].apiKey', GlobalVariable.userApiKey)
WS.verifyElementPropertyValue(responseApiKey, '[0].isActive', GlobalVariable.apiKeyStatus)
GlobalVariable.apiKeyId = (WS.getElementPropertyValue(responseApiKey, '[0].id'))
WebUI.comment('API KEY ID : ' + GlobalVariable.apiKeyId)