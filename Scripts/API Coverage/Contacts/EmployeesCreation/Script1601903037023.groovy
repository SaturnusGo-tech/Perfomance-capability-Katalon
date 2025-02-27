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

// Create new Contact and save Id
WebUI.comment('TEST CASE : Create employee')
response = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Customer/Employees/EmployeesCreate', [
	("contactName") : GlobalVariable.contactName,
	("firstName") : GlobalVariable.firstName,
	("lastName") : GlobalVariable.lastName,
	]))


// STEP | Parse request and save employee id to the GlobalVariable
def responseJson = new JsonSlurper().parseText(response.getResponseBodyContent())
GlobalVariable.employeeId = responseJson.id
WebUI.comment("Employee ID IS : " + GlobalVariable.employeeId)