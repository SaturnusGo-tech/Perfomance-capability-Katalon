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


//'CREATE KATALON OPERATOR @SUPPLIER PORTAL'
//operatorCreate = WS.callTestCase(findTestCase('Test Cases/KatalonSampleDataOperations/KatalonOperatorCreate'), null)


WebUI.comment('TEST CASE: VENDOR CREATE AND UPDATE')


'CREATE COMISSION AND GET ID'
comissionCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.MarketplaceVendorExtension/feeCreate',[
	('feeName') : GlobalVariable.katalonComissionName
	]))
comissionId = WS.getElementPropertyValue(comissionCreate, 'id')


'CREATE KATALON VENDOR(On Supplier portal)'
GlobalVariable.katalonVendorEmail  = new Random().nextInt(100)+'katalon@mail.com'
vendorCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.MarketplaceVendorExtension/sellerCreate',[
	('sellerName') : GlobalVariable.katalonVendorName,
	('comissionFeeId') : comissionId,
	('email') : GlobalVariable.katalonVendorEmail
	]))
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)


'GET KATALON VENDOR USERID'
userSearch = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/UserSearch', [
	('searchPhrase') : GlobalVariable.katalonVendorEmail
	]))
GlobalVariable.katalonVendorUserId = WS.getElementPropertyValue(userSearch, 'users[0].id')


'SEARCH FOR THE VENDOR TO VERIFY IT WAS CREATED AND GET ID'
vendorSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.MarketplaceVendorExtension/sellerSearch',[
	('keyword') : GlobalVariable.katalonVendorName
	]))
WS.verifyElementPropertyValue(vendorSearch, 'results[0].name', GlobalVariable.katalonVendorName)
GlobalVariable.katalonVendorId = WS.getElementPropertyValue(vendorSearch, 'results[0].id')


'GET TOKEN TO UPDATE VENDOR'//for some reason can only be authorized via token, no  apiKey auth. Bug mb?
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),
	null)


'UPDATE VENDORS APPROVAL POLICY'
approvalPolicy = 'Auto'
vendorUpdate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.MarketplaceVendorExtension/sellerUpdate',[
	('sellerId') : GlobalVariable.katalonVendorId,
	('commissionFeeId'): comissionId,
	('name'): GlobalVariable.katalonVendorName,
	('approvalPolicy') : approvalPolicy,
	('registrationId') : GlobalVariable.katalonVendorRegistrationId
	]))


'VERIFY VENDOR IS UPDATED'
vendorSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.MarketplaceVendorExtension/sellerSearch',[
	('keyword') : GlobalVariable.katalonVendorName
	]))
WS.verifyElementPropertyValue(vendorSearch, 'results[0].approvalPolicy', approvalPolicy)


'VERIFY INVITATION IS SENT'//the last 
journal = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Notifications/notificationsJournalSearch'))
notificationDate = WS.getElementPropertyValue(journal, 'results[0].createdDate')
status = WS.getElementPropertyValue(journal, 'results[0].status')
attemptCount = WS.getElementPropertyValue(journal, 'results[0].sendAttemptCount')
recepientEmail = WS.getElementPropertyValue(journal, 'results[0].to')
assert status == 'Error' || status == 'Pending'
assert attemptCount == 2
assert recepientEmail  == GlobalVariable.katalonVendorEmail


'SET USER PASSWORD FOR THE CREATED VENDOR'
passwordReset = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/UserPassword/UserPasswordReset',[
	('newPassword') : GlobalVariable.katalonVendorPassword,
	('userName') : GlobalVariable.katalonVendorEmail
	]))
WS.verifyElementPropertyValue(passwordReset, 'succeeded', true)
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),
	null)


