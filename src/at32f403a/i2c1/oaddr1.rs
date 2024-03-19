#[doc = "Register `OADDR1` reader"]
pub type R = crate::R<OADDR1_SPEC>;
#[doc = "Register `OADDR1` writer"]
pub type W = crate::W<OADDR1_SPEC>;
#[doc = "Field `ADDR1` reader - Own address 1"]
pub type ADDR1_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR1` writer - Own address 1"]
pub type ADDR1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Address mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR1MODE_A {
    #[doc = "0: 7-bit slave address"]
    Add7 = 0,
    #[doc = "1: 10-bit slave address"]
    Add10 = 1,
}
impl From<ADDR1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR1MODE` reader - Address mode"]
pub type ADDR1MODE_R = crate::BitReader<ADDR1MODE_A>;
impl ADDR1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR1MODE_A {
        match self.bits {
            false => ADDR1MODE_A::Add7,
            true => ADDR1MODE_A::Add10,
        }
    }
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDR1MODE_A::Add7
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDR1MODE_A::Add10
    }
}
#[doc = "Field `ADDR1MODE` writer - Address mode"]
pub type ADDR1MODE_W<'a, REG> = crate::BitWriter<'a, REG, ADDR1MODE_A>;
impl<'a, REG> ADDR1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR1MODE_A::Add7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR1MODE_A::Add10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Own address 1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Address mode"]
    #[inline(always)]
    pub fn addr1mode(&self) -> ADDR1MODE_R {
        ADDR1MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR1")
            .field("addr1mode", &format_args!("{}", self.addr1mode().bit()))
            .field("addr1", &format_args!("{}", self.addr1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OADDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Own address 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<OADDR1_SPEC> {
        ADDR1_W::new(self, 0)
    }
    #[doc = "Bit 15 - Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn addr1mode(&mut self) -> ADDR1MODE_W<OADDR1_SPEC> {
        ADDR1MODE_W::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR1_SPEC;
impl crate::RegisterSpec for OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr1::R`](R) reader structure"]
impl crate::Readable for OADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr1::W`](W) writer structure"]
impl crate::Writable for OADDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for OADDR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
