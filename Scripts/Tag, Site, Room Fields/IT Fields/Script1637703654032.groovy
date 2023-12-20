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

WebUI.callTestCase(findTestCase('Logging/Login - Admin'), [('url') : GlobalVariable.url, ('admin') : GlobalVariable.admin
        , ('password') : GlobalVariable.adminpass], FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementNotPresent(findTestObject('Ticket Perf/svg_Spinner'), 20)

WebUI.click(findTestObject('Create Ticket/button/button_Create Ticket'))

for (int i = 0; i < 5; i++) {
    WebUI.click(findTestObject('Create Ticket/button/div_Crown Bug PT'))

    WebUI.setText(findTestObject('IT Fields/input_Tag Number_config.name'), tag)

    WebUI.click(findTestObject('IT Fields/svg_Tag Number_svg-inline--fa fa-check-squa_80ea2c'))

    WebUI.delay(10)

    room = WebUI.getText(findTestObject('IT Fields/fields/input_Room_Field'))

    WebUI.verifyMatch(room, GlobalVariable.room, true)

    WebUI.verifyElementText(findTestObject('IT Fields/fields/input_Site_Field'), site)

    WebUI.refresh(FailureHandling.STOP_ON_FAILURE)
}

WebUI.click(findTestObject('IT Fields/button_Cancel'))

