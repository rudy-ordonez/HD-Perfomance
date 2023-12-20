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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Logging/Login - User'), [('url') : GlobalVariable.url, ('user') : GlobalVariable.user, ('password') : GlobalVariable.adminpass], 
    FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementNotPresent(findTestObject('Ticket Perf/svg_Spinner'), 15)

WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/h1_Create Ticket'))

WebUI.waitForElementNotPresent(findTestObject('Ticket Perf/svg_Spinner'), 65)

WebUI.click(findTestObject('IT Fields - User/buttons/div_A - Testing PT'))

WebUI.setText(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/input_Summary_summary'), 
    'AM Fields Negative Test')

WebUI.setText(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/textarea_Description_description'), 
    'AM Fields Negative Test')

WebUI.click(findTestObject('IT Fields - User/buttons/Next'))

'No tag entered'
WebUI.click(findTestObject('IT Fields - User/buttons/svg_Disabled Next'))

WebUI.verifyElementNotPresent(findTestObject('IT Fields - User/buttons/h2_Whats the best way to reach you'), 4, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/a_Clickheretorecordittothisticket'))

WebUI.setText(findTestObject('Object Repository/IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/input_TagNumber_assetTagNumber'), 
    '534513')

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Verify Tag'))

WebUI.click(findTestObject('IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/button_Yes'))

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Device Details Next'))

site = WebUI.getText(findTestObject('IT Fields - User/AM fields/input_Site_field'))

WebUI.verifyMatch(site, '816 Client Support Services', true)

'Clear Site field'
WebUI.click(findTestObject('IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/span_Site_Clear'))

WebUI.verifyElementText(findTestObject('IT Fields - User/buttons/div_A Site is required'), 'A Site is required.', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('IT Fields - User/buttons/div_A Room is required'), 'A Room is required.', FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Disabled Submit'))

WebUI.verifyElementNotPresent(findTestObject('IT Fields - User/buttons/h3_Ticket Submitted modal'), 5)

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Previous'))

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Device Details Next'))

'Site and Room blank'
WebUI.click(findTestObject('IT Fields - User/Page_GetHelp 5.1.4 - User Dashboard/span_Site_Clear'))

WebUI.setText(findTestObject('IT Fields - User/AM fields/input_Site_field'), Keys.chord(Keys.SPACE, Keys.SPACE, Keys.TAB))

WebUI.click(findTestObject('IT Fields - User/buttons/svg_Disabled Submit'))

WebUI.verifyElementText(findTestObject('IT Fields - User/buttons/div_A Site is required'), 'A Site is required.', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('IT Fields - User/buttons/div_A Room is required'), 'A Room is required.', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementNotPresent(findTestObject('IT Fields - User/buttons/h3_Ticket Submitted modal'), 5)

WebUI.setText(findTestObject('IT Fields - User/AM fields/input_Room_field'), '123')

