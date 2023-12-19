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
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import groovy.time.TimeCategory as TimeCategory
import groovy.time.TimeDuration as TimeDuration

f = new File('C://Temp/login_perf.csv')

for (int i = 0; i < 5; i++) {
    WebUI.openBrowser('')

    WebUI.navigateToUrl(url)

    WebUI.click(findTestObject('Help Desk/Login/button_Sign In'))

    WebUI.setText(findTestObject('Help Desk/Login/input_Username_login'), username)

    WebUI.setText(findTestObject('Help Desk/Login/input_Password_pwd'), password)

    start = new Date()

    WebUI.click(findTestObject('Help Desk/Login/button_Log On'))

    WebUI.waitForElementClickable(findTestObject('Help Desk/Login/dropdown/span_--- Sort By ---dropdown'), 10)

    end = new Date()

    TimeDuration td = TimeCategory.minus(end, start)

    (f << '\n') << td // Appends result to file

    WebUI.delay(3)

    WebUI.click(findTestObject('Help Desk/Logout/a_Rudy Ordonez'))

    WebUI.click(findTestObject('Help Desk/Logout/a_Log Out'))

    WebUI.closeBrowser()
}