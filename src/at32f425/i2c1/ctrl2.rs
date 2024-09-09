#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `SADDR` reader - Slave address"]
pub type SADDR_R = crate::FieldReader<u16>;
#[doc = "Field `SADDR` writer - Slave address"]
pub type SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DIR` reader - Master data transmission direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Master data transmission direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR10` reader - Host send 10-bit address mode enable"]
pub type ADDR10_R = crate::BitReader;
#[doc = "Field `ADDR10` writer - Host send 10-bit address mode enable"]
pub type ADDR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READH10` reader - 10-bit address header read enable"]
pub type READH10_R = crate::BitReader;
#[doc = "Field `READH10` writer - 10-bit address header read enable"]
pub type READH10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSTART` reader - Generate start condition"]
pub type GENSTART_R = crate::BitReader;
#[doc = "Field `GENSTART` writer - Generate start condition"]
pub type GENSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSTOP` reader - Generate stop condition"]
pub type GENSTOP_R = crate::BitReader;
#[doc = "Field `GENSTOP` writer - Generate stop condition"]
pub type GENSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKEN` reader - Not acknowledge enable"]
pub type NACKEN_R = crate::BitReader;
#[doc = "Field `NACKEN` writer - Not acknowledge enable"]
pub type NACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT` reader - Transmit data counter"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Transmit data counter"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLDEN` reader - Send data reload mode enable"]
pub type RLDEN_R = crate::BitReader;
#[doc = "Field `RLDEN` writer - Send data reload mode enable"]
pub type RLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASTOPEN` reader - Automatically send stop condition enable"]
pub type ASTOPEN_R = crate::BitReader;
#[doc = "Field `ASTOPEN` writer - Automatically send stop condition enable"]
pub type ASTOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECTEN` reader - Request PEC transmission enable"]
pub type PECTEN_R = crate::BitReader;
#[doc = "Field `PECTEN` writer - Request PEC transmission enable"]
pub type PECTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Master data transmission direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host send 10-bit address mode enable"]
    #[inline(always)]
    pub fn addr10(&self) -> ADDR10_R {
        ADDR10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header read enable"]
    #[inline(always)]
    pub fn readh10(&self) -> READH10_R {
        READH10_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generate start condition"]
    #[inline(always)]
    pub fn genstart(&self) -> GENSTART_R {
        GENSTART_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generate stop condition"]
    #[inline(always)]
    pub fn genstop(&self) -> GENSTOP_R {
        GENSTOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Not acknowledge enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Transmit data counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Send data reload mode enable"]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatically send stop condition enable"]
    #[inline(always)]
    pub fn astopen(&self) -> ASTOPEN_R {
        ASTOPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&self) -> PECTEN_R {
        PECTEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("pecten", &self.pecten())
            .field("astopen", &self.astopen())
            .field("rlden", &self.rlden())
            .field("cnt", &self.cnt())
            .field("nacken", &self.nacken())
            .field("genstop", &self.genstop())
            .field("genstart", &self.genstart())
            .field("readh10", &self.readh10())
            .field("addr10", &self.addr10())
            .field("dir", &self.dir())
            .field("saddr", &self.saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SADDR_W<CTRL2_SPEC> {
        SADDR_W::new(self, 0)
    }
    #[doc = "Bit 10 - Master data transmission direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CTRL2_SPEC> {
        DIR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Host send 10-bit address mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr10(&mut self) -> ADDR10_W<CTRL2_SPEC> {
        ADDR10_W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header read enable"]
    #[inline(always)]
    #[must_use]
    pub fn readh10(&mut self) -> READH10_W<CTRL2_SPEC> {
        READH10_W::new(self, 12)
    }
    #[doc = "Bit 13 - Generate start condition"]
    #[inline(always)]
    #[must_use]
    pub fn genstart(&mut self) -> GENSTART_W<CTRL2_SPEC> {
        GENSTART_W::new(self, 13)
    }
    #[doc = "Bit 14 - Generate stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn genstop(&mut self) -> GENSTOP_W<CTRL2_SPEC> {
        GENSTOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Not acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn nacken(&mut self) -> NACKEN_W<CTRL2_SPEC> {
        NACKEN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Transmit data counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CTRL2_SPEC> {
        CNT_W::new(self, 16)
    }
    #[doc = "Bit 24 - Send data reload mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<CTRL2_SPEC> {
        RLDEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatically send stop condition enable"]
    #[inline(always)]
    #[must_use]
    pub fn astopen(&mut self) -> ASTOPEN_W<CTRL2_SPEC> {
        ASTOPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Request PEC transmission enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecten(&mut self) -> PECTEN_W<CTRL2_SPEC> {
        PECTEN_W::new(self, 26)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
